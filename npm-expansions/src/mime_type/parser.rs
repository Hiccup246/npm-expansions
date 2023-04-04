// Inspiration for these functions is taken from https://www.xml.com/pub/a/2005/06/08/restful.html
use std::collections::HashMap;

type MimeType<'a> = (&'a str, &'a str, Option<HashMap<&'a str, &'a str>>);

/// A error for representing the failure to process a mime type
#[derive(Debug)]
pub struct InvalidMimeType;

/// Parses a mime type string slice into a tuple consisting of its type, subtype and parameters
///
/// # Arguments
///
/// * `mime_type` - A mime type represented as a string slice
///
/// # Examples
///
/// ```
/// use npm_expansions::mime_type::parser::parse_mime_type;
///
/// let parsed_mime_type = parse_mime_type("text/html");
///
/// assert_eq!(parsed_mime_type.unwrap(), ("text", "html", None));
/// ```
///
/// # Failures
///
/// The function fails if the given mime type is invalid (correctness based on https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
///
/// ```rust,should_error
/// // fails if given a malformed mime type
/// use npm_expansions::mime_type::parser::parse_mime_type;
///
/// parse_mime_type("text/");
/// ```
pub fn parse_mime_type(mime_type: &str) -> Result<MimeType, InvalidMimeType> {
    let parts: Vec<&str> = mime_type.trim().split(';').collect();

    let parameters = match parts.get(1..) {
        Some(params) => parse_mime_parameters(params.to_vec()),
        _ => Ok(None),
    }?;

    let (primary_type, subtype) = parts
        .first()
        .ok_or(InvalidMimeType)?
        .split_once('/')
        .ok_or(InvalidMimeType)?;

    if primary_type.is_empty() || subtype.is_empty() {
        Err(InvalidMimeType)
    } else {
        Ok((primary_type, subtype, parameters))
    }
}

fn parse_mime_parameters(
    parameters: Vec<&str>,
) -> Result<Option<HashMap<&str, &str>>, InvalidMimeType> {
    let key_value_parameters: Result<Vec<(&str, &str)>, InvalidMimeType> = parameters
        .iter()
        .map(|val| val.split_once('=').ok_or(InvalidMimeType))
        .collect();

    match key_value_parameters? {
        params if !params.is_empty() => Ok(Some(HashMap::from_iter(params))),
        _ => Ok(None),
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
    fn mime_with_multiple_params() {
        let parsed_mime = parse_mime_type("application/signed-exchange;v=b3;q=0.7").unwrap();
        let full_unwrapped = (parsed_mime.0, parsed_mime.1, parsed_mime.2.unwrap());

        assert_eq!(
            full_unwrapped,
            (
                "application",
                "signed-exchange",
                HashMap::from([("v", "b3"), ("q", "0.7")])
            )
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
