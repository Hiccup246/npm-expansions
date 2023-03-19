use crate::mime_type_parser;

#[derive(Debug, Clone)]
pub struct SupportedMimeTypeError;
pub struct InvalidAcceptHeaderError;
pub trait ParseError {}

impl ParseError for InvalidAcceptHeaderError {}
impl ParseError for SupportedMimeTypeError {}

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
pub fn best_match(
    supported_mime_types: Vec<&str>,
    accept_header: &str,
) -> Result<String, Box<dyn ParseError>> {
    let parsed_accept_headers: Result<
        Vec<(&str, &str, f32)>,
        mime_type_parser::MimeTypeParseError,
    > = accept_header
        .split(",")
        .map(|header_str| fitness_ready_mime_type(header_str))
        .collect();

    if let Ok(parsed_accept_headers) = parsed_accept_headers {
        let mut weighted_matches: Result<Vec<(f32, &str)>, mime_type_parser::MimeTypeParseError> =
            supported_mime_types
                .iter()
                .map(|mime_type| {
                    fitness_of_mime_type(mime_type, &parsed_accept_headers)
                        .and_then(|val| Ok((val, *mime_type)))
                })
                .collect();

        if let Ok(weighted_matches) = weighted_matches {
            weighted_matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let final_match = weighted_matches.get(weighted_matches.len() - 1).unwrap();

            if final_match.0 != 0.0 {
                Ok(final_match.1.to_string())
            } else {
                Ok("".to_string())
            }
        } else {
            return Err(Box::new(SupportedMimeTypeError));
        }
    } else {
        return Err(Box::new(InvalidAcceptHeaderError));
    }
}

pub fn fitness_ready_mime_type(
    mime_type: &str,
) -> Result<(&str, &str, f32), mime_type_parser::MimeTypeParseError> {
    let (mime_type, subtype, parameter) = mime_type_parser::parse_mime_type(mime_type)?;
    let mut quality = 1.0;

    if let Some(parameter_hash) = parameter {
        let parsed_quality = parameter_hash
            .get("q")
            .unwrap_or(&"")
            .parse()
            .unwrap_or(1.0);

        if parsed_quality < 0.0 || parsed_quality > 1.0 {
            quality = 1.0
        };
    }

    Ok((mime_type, subtype, quality))
}

pub fn fitness_of_mime_type(
    mime_type: &str,
    mime_range: &Vec<(&str, &str, f32)>,
) -> Result<f32, mime_type_parser::MimeTypeParseError> {
    let (mime_type, mime_subtype, mime_quality) = fitness_ready_mime_type(mime_type)?;
    let mut best_fitness = -1.0;
    let mut best_mime_type_quality = 0.0;

    for (range_type, range_subtype, range_quality) in mime_range {
        if *range_type == mime_type || *range_type == "*" {
            if *range_subtype == mime_subtype || *range_subtype == "*" {
                let mut fitness = -1.0;

                if *range_type == mime_type {
                    fitness += 100.0
                } else {
                    fitness += 0.0
                };

                if *range_subtype == mime_subtype {
                    fitness += 10.0
                } else {
                    fitness += 0.0
                };

                fitness += range_quality;

                if fitness > best_fitness {
                    best_fitness = fitness;
                    best_mime_type_quality = *range_quality;
                }
            }
        }
    }

    Ok(best_mime_type_quality)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fitness_of_mime_type_tests {
        use super::*;

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
    }

    mod best_match_tests {
        use super::*;

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
}
