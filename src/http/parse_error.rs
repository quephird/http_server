use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidUTF8Encoding,
    InvalidHTTPVersion,
    InvalidHTTPMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request!",
            Self::InvalidUTF8Encoding => "Invalid UTF8 encoding!",
            Self::InvalidHTTPVersion => "Invalid HTTP version sent!",
            Self::InvalidHTTPMethod => "Invalid HTTP method requested!",
        }
    }
}

// This is required to satisfy the Error trait below
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message())
    }
}

// Empty implementation
impl Error for ParseError {}