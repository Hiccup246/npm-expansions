// Inspiration for these functions is taken from https://www.xml.com/pub/a/2005/06/08/restful.html
use crate::mime_type_parser;
use std::fmt;

#[derive(Debug)]
pub struct SupportedMimeTypeError;

#[derive(Debug)]
pub struct InvalidAcceptHeaderError;

pub trait ParseError {}

impl fmt::Debug for dyn ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error Occured")
    }
}

impl ParseError for InvalidAcceptHeaderError {}
impl ParseError for SupportedMimeTypeError {}

/// Returns the most appropriate mime type given a list of desired types and an accept header
///
/// # Arguments
///
/// * `supported_mime_types` - A vector of desired mime types represented as string slices
/// * `accept_header` - A string slice representing a given accept header
///
/// # Examples
///
/// ```
/// let best_mime_match = best_match(Vec::from(["application/json"], "text/plain, application/json"));
/// assert_eq!(best_mime_match.unwrap(), "application/json".to_string());
/// ```
///
/// # Failures
///
/// The function fails if any of the supported mime types or the accept header is malformed
///
/// ```rust,should_error
/// // fails if given malformed supported mime types or the accept header
/// best_match(Vec::from(["application/"], "/plain"))
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
        let weighted_matches: Result<Vec<(f32, &str)>, mime_type_parser::MimeTypeParseError> =
            supported_mime_types
                .iter()
                .map(|mime_type| {
                    fitness_of_mime_type(mime_type, &parsed_accept_headers)
                        .and_then(|val| Ok((val, *mime_type)))
                })
                .collect();

        if let Ok(mut ok_weighted_matches) = weighted_matches {
            ok_weighted_matches.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            let final_match = ok_weighted_matches
                .get(ok_weighted_matches.len() - 1)
                .unwrap();

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

        if parsed_quality >= 0.0 || parsed_quality <= 1.0 {
            quality = parsed_quality;
        }
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
                let mut fitness = 0.0;

                if *range_type == mime_type {
                    fitness += 100.0
                }

                if *range_subtype == mime_subtype {
                    fitness += 10.0
                }

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
        fn exact_match() {
            assert_eq!(
                fitness_of_mime_type(
                    "text/plain",
                    &Vec::from([("text", "plain", 1.0), ("text", "html", 1.0)])
                )
                .unwrap(),
                1.0
            );
        }

        #[test]
        fn no_match() {
            assert_eq!(
                fitness_of_mime_type("text/plain", &Vec::from([("text", "html", 1.0)])).unwrap(),
                0.0
            );
        }

        #[test]
        fn half_match() {
            assert_eq!(
                fitness_of_mime_type(
                    "text/plain",
                    &Vec::from([("text", "*", 1.0), ("application", "json", 1.0)])
                )
                .unwrap(),
                1.0
            );
        }

        #[test]
        fn quality_match() {
            assert_eq!(
                fitness_of_mime_type(
                    "text/plain",
                    &Vec::from([("text", "plain", 0.5), ("text", "*", 1.0)])
                )
                .unwrap(),
                0.5
            );
        }

        #[test]
        fn invalid_supported_mime_type() {
            assert!(fitness_of_mime_type(
                "text/",
                &Vec::from([("text", "plain", 0.5), ("text", "*", 1.0)])
            )
            .is_err());
        }

        #[test]
        fn invalid_mime_range() {
            assert_eq!(
                fitness_of_mime_type(
                    "text/plain",
                    &Vec::from([("text", "", 0.5), ("", "*", 1.0)])
                )
                .unwrap(),
                0.0
            );
        }
    }

    mod best_match_tests {
        use super::*;

        #[test]
        fn exact_match() {
            assert_eq!(
                best_match(
                    Vec::from(["text/plain", "text/*"]),
                    "application/json, text/plain"
                )
                .unwrap(),
                "text/plain"
            );
        }

        #[test]
        fn generic_type_match() {
            assert_eq!(
                best_match(
                    Vec::from(["text/plain", "text/*"]),
                    "application/json, */plain"
                )
                .unwrap(),
                "text/plain"
            );
        }

        #[test]
        fn generic_subtype_match() {
            assert_eq!(
                best_match(
                    Vec::from(["text/plain", "text/*"]),
                    "application/json, text/*"
                )
                .unwrap(),
                "text/*"
            );
        }

        #[test]
        fn no_match() {
            assert_eq!(
                best_match(
                    Vec::from(["text/plain", "text/*"]),
                    "application/json, image/jpeg"
                )
                .unwrap(),
                ""
            );
        }

        #[test]
        fn no_supported_mime_types() {
            assert_eq!(
                best_match(Vec::from([]), "application/json, image/jpeg").unwrap(),
                ""
            );
        }

        #[test]
        fn no_accept_header() {
            assert_eq!(best_match(Vec::from(["text/plain", ""]), "").unwrap(), "");
        }

        #[test]
        fn invalid_supported_mime_type() {
            assert!(best_match(Vec::from(["text/"]), "application/json, image/jpeg").is_err());
        }

        #[test]
        fn invalid_accept_header() {
            assert!(best_match(Vec::from(["text/plain"]), "application/, image/jpeg").is_err());
        }
    }
}
