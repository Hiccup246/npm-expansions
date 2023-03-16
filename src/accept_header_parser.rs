// Take in an acceptance string
// Return a hashmap with name and priority
pub fn parse_mime_type<'a>(mime_type: &'a str) -> Option<(&'a str, &'a str, i32)> {
    let parts: Vec<&str> = mime_type.trim().split(";").collect();

    let q_value: Vec<&str> = parts.get(1).unwrap_or(&"q=1").split("=").collect();
    let mut parsed_q_value = q_value.get(1).unwrap_or(&"1").parse().unwrap_or(1);

    if parsed_q_value > 1 || parsed_q_value < 0 {
        parsed_q_value = 1;
    }

    let type_breakdown: Vec<&str> = parts.get(0)?.split("/").collect();
    let atype = type_breakdown.get(0)?;
    let subtype = type_breakdown.get(1)?;

    // (application, json, 1)
    Some((atype, subtype, parsed_q_value))
}

pub fn fitness_of_mime_type(mime_type: &str, mime_range: &Vec<(&str, &str, i32)>) -> i32 {
    let (trarget_type, target_subtype, target_priority) = parse_mime_type(mime_type).unwrap();
    let mut best_fitness = -1;
    let mut best_fit_q = 0;

    for (parsed_type, parsed_subtype, parsed_priority) in mime_range {
        if *parsed_type == trarget_type || *parsed_type == "*" {
            if *parsed_subtype == target_subtype || *parsed_subtype == "*" {
                let mut fitness = if *parsed_type == trarget_type { 100 } else { 0 };
                fitness += if *parsed_subtype == target_subtype {
                    10
                } else {
                    0
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

pub fn best_match(supported: Vec<String>, header: String) -> String {
    let parsed_header = &header
        .split(",")
        .map(|header_str| parse_mime_type(header_str).unwrap())
        .collect();
    let mut weighted_matches: Vec<(i32, &String)> = supported
        .iter()
        .map(|mime_type| (fitness_of_mime_type(mime_type, parsed_header), mime_type))
        .collect();

    weighted_matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    weighted_matches
        .get(weighted_matches.len() - 1)
        .unwrap()
        .1
        .to_string()
}
