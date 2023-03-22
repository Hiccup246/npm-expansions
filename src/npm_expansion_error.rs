#[derive(Debug)]
pub struct NpmExpansionsError {
    kind: NpmErrorKind,
}

// internal server error
// client related issues
#[derive(Debug)]
pub enum NpmErrorKind {
    InvalidHeader,
    TooManyHeaders,
    RequestParseError,
    SupportedMimeTypeError,
    InternalServerError,
}

impl NpmExpansionsError {
    pub fn new(kind: NpmErrorKind) -> NpmExpansionsError {
        NpmExpansionsError { kind }
    }

    pub fn kind(&self) -> &NpmErrorKind {
        &self.kind
    }
}
