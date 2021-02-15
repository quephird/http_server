use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn description(&self) -> &str {
        match self {
            Self::Ok => "OK 👍",
            Self::BadRequest => "Bad request 🙍‍♀️",
            Self::NotFound => "Not found 🤷‍♀️",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u16)
    }
}