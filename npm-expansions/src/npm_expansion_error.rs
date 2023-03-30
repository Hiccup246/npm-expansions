/// Error type of the server
///
/// In order for the server to process different types of errors and
/// return appropriate responses i.e. 400, 500 the error has a kind attribute
#[derive(Debug)]
pub struct NpmExpansionsError {
    kind: NpmErrorKind,
}

/// Enum for the different kinds of errors the server can produce
#[derive(Debug)]
pub enum NpmErrorKind {
    InvalidHeader,
    TooManyHeaders,
    RequestParseError,
    SupportedMimeTypeError,
    InternalServerError,
    InvalidRequestStatusLine,
}

impl NpmExpansionsError {
    pub fn new(kind: NpmErrorKind) -> NpmExpansionsError {
        NpmExpansionsError { kind }
    }

    pub fn kind(&self) -> &NpmErrorKind {
        &self.kind
    }
}
