use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MimeTypeParseError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for MimeTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Mime Type")
    }
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```
pub fn best_match(supported: Vec<String>, header: &String) -> String {
    if header.is_empty() || supported.len() == 0 {
        return "".to_string();
    }

    let parsed_header = &header
        .split(",")
        .map(|header_str| fitness_ready_mime_type(header_str).unwrap())
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

pub fn fitness_ready_mime_type(mime_type: &str) -> Result<(&str, &str, f32), MimeTypeParseError> {
    let (mime_type, subtype, parameter) = parse_mime_type(mime_type)?;
    let mut quality = 1.0;
    
    if let Some(parameter_hash) = parameter {
        let parsed_quality = parameter_hash.get("q").unwrap_or(&"").parse().unwrap_or(1.0);

        if parsed_quality < 0.0 || parsed_quality > 1.0 {
            quality = 1.0
        };   
    }

    Ok((mime_type, subtype, quality))
}

pub fn fitness_of_mime_type(mime_type: &str, mime_range: &Vec<(&str, &str, f32)>) -> f32 {
    let (trarget_type, target_subtype, target_priority) = fitness_ready_mime_type(mime_type).unwrap();
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

pub fn parse_mime_type<'a>(
    mime_type: &'a str,
) -> Result<(&'a str, &'a str, Option<HashMap<&str, &str>>), MimeTypeParseError> {
    let parts: Vec<&str> = mime_type.trim().split(";").collect();    

    let parameters:Option<HashMap<&str, &str>> = if let Some(parameter_values) = parts.get(1..) {
        let split_parameters:Result<Vec<(&str, &str)>, MimeTypeParseError> = parameter_values.into_iter()
            .map(|val| val.split_once("=").ok_or(MimeTypeParseError)).collect();

        if let Ok(split_parameters) = split_parameters {
            Some(HashMap::from_iter(split_parameters))
        } else {
            None
        } 
    } else {
        None
    };

    let mime_type = match parts.get(0) {
        Some(mime_type_value) => mime_type_value.split_once("/"),
        _ => None,
    };

    if let Some(mime_type) = mime_type {
        Ok((mime_type.0, mime_type.1, parameters))
    } else {
        Err(MimeTypeParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_parse_mime_type {
        use super::*;

        #[test]
        fn valid_mime() {
            assert_eq!(
                parse_mime_type("application/json").unwrap(),
                ("application", "json", 1.0)
            );
        }

        #[test]
        fn wild_card_type_mime() {
            assert_eq!(parse_mime_type("*/*").unwrap(), ("*", "*", 1.0));
        }

        #[test]
        fn wild_card_subtype_mime() {
            assert_eq!(parse_mime_type("text/*").unwrap(), ("text", "*", 1.0));
        }

        #[test]
        fn mime_with_quality() {
            assert_eq!(
                parse_mime_type("text/plain;q=0.8").unwrap(),
                ("text", "plain", 0.8)
            );
        }

        #[test]
        fn no_type_mime() {
            assert_eq!(parse_mime_type("/plain"), None);
        }

        #[test]
        fn no_subtype_mime() {
            assert_eq!(parse_mime_type("text/"), None);
        }

        #[test]
        fn malformed_quality_mime() {
            assert_eq!(parse_mime_type("text/plain;q0.8"), None);
        }
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
