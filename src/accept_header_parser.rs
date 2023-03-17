// Take in an acceptance string
// Return a hashmap with name and priority
pub fn parse_mime_type<'a>(mime_type: &'a str) -> Option<(&'a str, &'a str, f32)> {
    let parts: Vec<&str> = mime_type.trim().split(";").collect();

    let q_value: Vec<&str> = parts.get(1).unwrap_or(&"q=1").split("=").collect();

    if q_value.len() != 2 {
        return None;
    }

    let mut parsed_q_value = q_value.get(1).unwrap_or(&"1").parse().unwrap_or(1.0);

    if parsed_q_value > 1.0 || parsed_q_value < 0.0 {
        parsed_q_value = 1.0;
    }

    let type_breakdown: Vec<&str> = parts.get(0)?.split("/").collect();
    let atype = type_breakdown.get(0)?;
    let subtype = type_breakdown.get(1)?;

    if subtype.is_empty() || atype.is_empty() {
        return None;
    }
    // (application, json, 1)
    Some((atype, subtype, parsed_q_value))
}

pub fn fitness_of_mime_type(mime_type: &str, mime_range: &Vec<(&str, &str, f32)>) -> f32 {
    let (trarget_type, target_subtype, target_priority) = parse_mime_type(mime_type).unwrap();
    let mut best_fitness = -1.0;
    let mut best_fit_q = 0.0;

    for (parsed_type, parsed_subtype, parsed_priority) in mime_range {
        if *parsed_type == trarget_type || *parsed_type == "*" {
            if *parsed_subtype == target_subtype || *parsed_subtype == "*" {
                let mut fitness = -1.0;

                if *parsed_type == trarget_type {
                    fitness += 100.0
                } else {
                    fitness += 0.0
                };

                if *parsed_subtype == target_subtype {
                    fitness += 10.0
                } else {
                    fitness += 0.0
                };
                fitness += target_priority;

                if fitness > best_fitness {
                    best_fitness = fitness;
                    best_fit_q = *parsed_priority;
                }
            }
        }
    }

    best_fit_q
}

pub fn best_match(supported: Vec<String>, header: &String) -> String {
    if header.is_empty() || supported.len() == 0 {
        return "".to_string();
    }

    let parsed_header = &header
        .split(",")
        .map(|header_str| parse_mime_type(header_str).unwrap())
        .collect();
    let mut weighted_matches: Vec<(f32, &String)> = supported
        .iter()
        .map(|mime_type| (fitness_of_mime_type(mime_type, parsed_header), mime_type))
        .collect();

    weighted_matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let final_match = weighted_matches.get(weighted_matches.len() - 1).unwrap();

    if final_match.0 != 0.0 && final_match.1 != "" {
        final_match.1.to_string()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mime_type_t() {
        assert_eq!(
            parse_mime_type("application/json").unwrap(),
            ("application", "json", 1.0)
        );
    }

    #[test]
    fn parse_mime_type_wild_card() {
        assert_eq!(parse_mime_type("*/*").unwrap(), ("*", "*", 1.0));
    }

    #[test]
    fn parse_mime_type_sub_type_wild_card() {
        assert_eq!(parse_mime_type("text/*").unwrap(), ("text", "*", 1.0));
    }

    #[test]
    fn parse_mime_type_quality() {
        assert_eq!(
            parse_mime_type("text/plain;q=0.8").unwrap(),
            ("text", "plain", 0.8)
        );
    }

    #[test]
    fn parse_mime_type_no_type() {
        assert_eq!(parse_mime_type("/plain"), None);
    }

    #[test]
    fn parse_mime_type_no_subtype() {
        assert_eq!(parse_mime_type("text/"), None);
    }

    #[test]
    fn parse_mime_type_malformed_quality() {
        assert_eq!(parse_mime_type("text/plain;q0.8"), None);
    }

    #[test]
    fn fitness_of_mime_type_exact_match() {
        assert_eq!(
            fitness_of_mime_type(
                "text/plain",
                &Vec::from([("text", "plain", 1.0), ("text", "html", 1.0)])
            ),
            1.0
        );
    }

    #[test]
    fn fitness_of_mime_type_no_match() {
        assert_eq!(
            fitness_of_mime_type("text/plain", &Vec::from([("text", "html", 1.0)])),
            0.0
        );
    }

    #[test]
    fn fitness_of_mime_type_half_match() {
        assert_eq!(
            fitness_of_mime_type(
                "text/plain",
                &Vec::from([("text", "*", 1.0), ("application", "json", 1.0)])
            ),
            1.0
        );
    }

    #[test]
    fn fitness_of_mime_type_quality_match() {
        assert_eq!(
            fitness_of_mime_type(
                "text/plain",
                &Vec::from([("text", "plain", 0.5), ("text", "*", 1.0)])
            ),
            0.5
        );
    }

    #[test]
    fn best_match_exact() {
        assert_eq!(
            best_match(
                Vec::from(["text/plain".to_string(), "text/*".to_string()]),
                &"application/json, text/plain".to_string()
            ),
            "text/plain".to_string()
        );
    }

    #[test]
    fn best_match_type_generic() {
        assert_eq!(
            best_match(
                Vec::from(["text/plain".to_string(), "text/*".to_string()]),
                &"application/json, */plain".to_string()
            ),
            "text/plain".to_string()
        );
    }

    #[test]
    fn best_match_subtype_generic() {
        assert_eq!(
            best_match(
                Vec::from(["text/plain".to_string(), "text/*".to_string()]),
                &"application/json, text/*".to_string()
            ),
            "text/*".to_string()
        );
    }

    #[test]
    fn best_match_no_match() {
        assert_eq!(
            best_match(
                Vec::from(["text/plain".to_string(), "text/*".to_string()]),
                &"application/json, image/jpeg".to_string()
            ),
            "".to_string()
        );
    }

    #[test]
    fn best_match_no_supported_types() {
        assert_eq!(
            best_match(Vec::from([]), &"application/json, image/jpeg".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn best_match_no_header() {
        assert_eq!(
            best_match(
                Vec::from(["text/plain".to_string(), "".to_string()]),
                &"".to_string()
            ),
            "".to_string()
        );
    }
}
