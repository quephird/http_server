use std::fs;

use super::method::Method;
use super::parse_error::ParseError;
use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;

pub trait Handler {
    fn handle_success(&mut self, request: &Request) -> Response;

    fn handle_failure(&mut self, error: &ParseError) -> Response;
}

pub struct HandlerImpl {
    pub public_path: String,
}

impl HandlerImpl {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path,
        }
    }

    fn read_file(&self, relative_path: &str) -> Option<String> {
        let absolute_path = format!("{}/{}", self.public_path, relative_path);
        fs::read_to_string(absolute_path).ok()
    }
}

impl Handler for HandlerImpl {
    fn handle_success(&mut self, request: &Request) -> Response {
        println!("Received a request: {:?}", request);

        match request.method {
            Method::GET =>
                match request.path {
                    "/" => Response {
                        status: StatusCode::Ok,
                        body: self.read_file("index.html"),
                    },
                    _ => Response {
                        status: StatusCode::NotFound,
                        body: self.read_file("404.html"),
                    },
                }
            _ => 
                Response {
                    status: StatusCode::NotFound,
                    body: Some(
                        "<html>
                        <body>;_; Method not supported</body>
                        </html".to_string()
                    ),
                },
        }
    }

    fn handle_failure(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response {
            status: StatusCode::BadRequest,
            body: Some("I am sorry, Danielle!!!".to_string()),
        }
    }
}