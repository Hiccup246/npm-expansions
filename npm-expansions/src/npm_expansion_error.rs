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
    /// A HTTP request has invalid headers
    InvalidRequestHeaders,
    /// A HTTP request more than 8000 bytes of headers
    TooManyRequestHeaders,
    /// A HTTP request contains invalid mime types
    InvalidRequestMimeType,
    /// A HTTP request has an invalid format
    InvalidHttpRequest,
    /// A mime type provided by the server is invalid
    SupportedMimeTypeError,
    /// The sever encountered an error when processing a HTTP request
    InternalServerError,
    /// A HTTP request has a route not supported by the server
    RouteNotFound,
}

impl NpmExpansionsError {
    /// Creates a error from a given NpmErrorKind and message
    pub fn new(kind: NpmErrorKind, message: &str) -> NpmExpansionsError {
        NpmExpansionsError {
            kind,
            message: message.to_string(),
        }
    }

    /// Creates a error from a NpmErrorKind
    pub fn from(kind: NpmErrorKind) -> NpmExpansionsError {
        NpmExpansionsError {
            kind,
            message: "An NPM expansion error occured".to_string(),
        }
    }

    /// Returns the NpmErrorKind of a NpmExpansionsError
    pub fn kind(&self) -> &NpmErrorKind {
        &self.kind
    }
}

impl fmt::Display for NpmExpansionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NpmExpansionsError of kind: {:?} and message: {}",
            self.kind(),
            self.message
        )
    }
}
