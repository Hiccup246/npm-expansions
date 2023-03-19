use std::{collections::HashMap, fmt};

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

pub fn parse_mime_type<'a>(
    mime_type: &'a str,
) -> Result<(&'a str, &'a str, Option<HashMap<&str, &str>>), MimeTypeParseError> {
    let parts: Vec<&str> = mime_type.trim().split(";").collect();

    let parameters: Option<HashMap<&str, &str>> = if let Some(parameter_values) = parts.get(1..) {
        let split_parameters: Result<Vec<(&str, &str)>, MimeTypeParseError> = parameter_values
            .into_iter()
            .map(|val| val.split_once("=").ok_or(MimeTypeParseError))
            .collect();

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

mod tests {
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
