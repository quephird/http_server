use std::convert::TryFrom;
use super::method::Method;

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
    headers: Vec<String>,
    body: Option<String>,
}        

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        Ok(
            Self {
                method: Method::GET,
                path: "/".to_string(),
                query_string: None,
                headers: vec![],
                body: None,
            }
        )
    }
}