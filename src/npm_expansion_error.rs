#[derive(Debug)]
pub struct NpmExpansionsError {
    kind: NpmErrorKind,
}

#[derive(Debug)]
pub enum NpmErrorKind {
    InvalidHeader,
    TooManyHeaders,
    RequestParseError,
}

impl NpmExpansionsError {
    pub fn new(kind: NpmErrorKind) -> NpmExpansionsError {
        NpmExpansionsError { kind }
    }

    pub fn kind(&self) -> &NpmErrorKind {
        &self.kind
    }
}
