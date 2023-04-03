use std::fmt;

/// Error type of the server
///
/// In order for the server to process different types of errors and
/// return appropriate responses i.e. 400, 500 the error has a kind attribute
#[derive(Debug)]
pub struct NpmExpansionsError {
    kind: NpmErrorKind,
    message: String,
}

/// Enum for the different kinds of errors the server can produce
#[derive(Debug)]
pub enum NpmErrorKind {
    InvalidHeader,
    TooManyHeaders,
    InvalidMimeType,
    RequestParseError,
    SupportedMimeTypeError,
    InternalServerError,
    NotFound,
}

impl NpmExpansionsError {
    pub fn new(kind: NpmErrorKind, message: &str) -> NpmExpansionsError {
        NpmExpansionsError {
            kind,
            message: message.to_string(),
        }
    }

    pub fn from(kind: NpmErrorKind) -> NpmExpansionsError {
        NpmExpansionsError {
            kind,
            message: "An NPM expansion error Occured".to_string(),
        }
    }

    pub fn kind(&self) -> &NpmErrorKind {
        &self.kind
    }
}

impl fmt::Display for NpmExpansionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
