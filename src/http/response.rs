use std::collections::HashMap;
use std::io::Result as IoResult;
use std::io::Write;

use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    // headers: Hashmap<String, String>,
    pub body: Option<String>,
}

impl Response {
    pub fn new(
        status: StatusCode,
        // headers: Hashmap<String, String>,
        body: Option<String>,
    ) -> Self {
        Self {
            status,
            // headers,
            body,
        }
    }

    // ACHTUNG! This function requires that the `stream` parameter
    // implements the `Write` trait.
    pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
        let body_string = match &self.body {
            Some(body) => body,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.description(),
            body_string,
        ) 
    }
}
