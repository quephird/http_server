use super::method::Method;
use super::parse_error::ParseError;
use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;

pub trait Handler {
    fn handle_success(&mut self, request: &Request) -> Response;

    fn handle_failure(&mut self, error: &ParseError) -> Response;
}

pub struct HandlerImpl;

impl Handler for HandlerImpl {
    fn handle_success(&mut self, request: &Request) -> Response {
        println!("Received a request: {:?}", request);

        match request.method {
            Method::GET =>
                match request.path {
                    "/" => Response {
                        status: StatusCode::Ok,
                        body: Some(
                            "<html>
                            <body>Welcome to the root, Danielle</body>
                            </html".to_string()
                        ),
                    },
                    _ => Response {
                        status: StatusCode::NotFound,
                        body: Some(
                            "<html>
                            <body>;_; Path not found</body>
                            </html".to_string()
                        ),
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