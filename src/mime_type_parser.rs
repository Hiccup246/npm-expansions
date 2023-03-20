// Inspiration for these functions is taken from https://www.xml.com/pub/a/2005/06/08/restful.html
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct MimeTypeParseError {
    pub failed_mime_type: String,
}

impl MimeTypeParseError {
    fn new(mime_type: String) -> MimeTypeParseError {
        MimeTypeParseError {
            failed_mime_type: mime_type,
        }
    }
}

impl fmt::Display for MimeTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!("{} is an invalid Mime Type", self.failed_mime_type);
        write!(f, "{message}")
    }
}

/// Parses a mime type string slice into a tuple consisting of its type, subtype and parameters
///
/// # Arguments
///
/// * `mime_type` - A mime type represented as a string slice
///
/// # Examples
///
/// ```
/// let parsed_mime_type = parse_mime_type("text/html");
/// assert_eq!(parsed_mime_type.unwrap(), ("text", "html", Option<HashMap::new()>));
/// ```
///
/// # Failures
///
/// The function fails if the given mime type is invalid (correctness based on https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
///
/// ```rust,should_error
/// // fails if given malformed supported mime types or the accept header
/// parse_mime_type("text/")
/// ```
pub fn parse_mime_type<'a>(
    mime_type: &'a str,
) -> Result<(&'a str, &'a str, Option<HashMap<&str, &str>>), MimeTypeParseError> {
    let parts: Vec<&str> = mime_type.trim().split(";").collect();

    let parameters: Result<Option<HashMap<&str, &str>>, MimeTypeParseError> = if let Some(parameter_values) = parts.get(1..) {
        let split_parameters: Result<Vec<(&str, &str)>, MimeTypeParseError> = parameter_values
            .into_iter()
            .map(|val| {
                val.split_once("=")
                    .ok_or(MimeTypeParseError::new(mime_type.to_string()))
            })
            .collect();

        if let Ok(split_parameters) = split_parameters {
            if split_parameters.is_empty() {
                Ok(None)
            } else {
                Ok(Some(HashMap::from_iter(split_parameters)))
            }
        } else {
            Err(split_parameters.err().unwrap())
        }
    } else {
        Ok(None)
    };

    let parsed_mime_type = match parts.get(0) {
        Some(mime_type_value) => mime_type_value.split_once("/"),
        _ => None,
    };

    if let Some(parsed_mime_type) = parsed_mime_type {

        if parameters.is_err() {
            Err(parameters.err().unwrap())
        } else {
            if parsed_mime_type.0.is_empty() || parsed_mime_type.1.is_empty() {
                Err(MimeTypeParseError::new(mime_type.to_string()))
            } else {
                Ok((parsed_mime_type.0, parsed_mime_type.1, parameters.unwrap()))
            }
        }
    } else {
        Err(MimeTypeParseError::new(mime_type.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_mime() {
        assert_eq!(
            parse_mime_type("application/json").unwrap(),
            ("application", "json", None)
        );
    }

    #[test]
    fn wild_card_type_mime() {
        assert_eq!(parse_mime_type("*/*").unwrap(), ("*", "*", None));
    }

    #[test]
    fn wild_card_subtype_mime() {
        assert_eq!(parse_mime_type("text/*").unwrap(), ("text", "*", None));
    }

    #[test]
    fn mime_with_quality() {
        let parsed_mime = parse_mime_type("text/plain;q=0.8").unwrap();
        let full_unwrapped = (parsed_mime.0, parsed_mime.1, parsed_mime.2.unwrap());

        assert_eq!(
            full_unwrapped,
            ("text", "plain", HashMap::from([("q", "0.8")]))
        );
    }

    #[test]
    fn no_type_mime() {
        println!("{}", parse_mime_type("/plain").is_err());
        assert!(parse_mime_type("/plain").is_err());
    }

    #[test]
    fn no_subtype_mime() {
        assert!(parse_mime_type("text/").is_err());
    }

    #[test]
    fn malformed_quality_mime() {
        assert!(parse_mime_type("text/plain;q0.8").is_err());
    }

    #[test]
    fn no_forward_slash() {
        assert!(parse_mime_type("text").is_err());
    }
}
