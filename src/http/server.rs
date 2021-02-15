use std::convert::TryFrom;
use std::io::Read;
use std::net::SocketAddrV4;
use std::net::TcpListener;

use super::request::Request;
use super::response::Response;
use super::status_code::StatusCode;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    fn handle_request(buffer: &mut [u8; 1024]) -> Response {
        match Request::try_from(&buffer[..]) {
            Ok(request) => {
                println!("Received a request: {:?}", request);
                Response {
                    status: StatusCode::Ok,
                    body: Some("Hello, Danielle!!!".to_string()),
                }
            }
            Err(e) => {
                println!("Failed to parse request: {}", e);
                Response {
                    status: StatusCode::BadRequest,
                    body: Some("I am sorry, Danielle!!!".to_string()),
                }
            }
        }
    }

    pub fn run(self) -> () {
        let socket = self.address.parse::<SocketAddrV4>().unwrap();
        println!("Listening on port {}...", socket.port());

        let listener = TcpListener::bind(socket).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _address)) => {
                    // We choose 1024 below just to be large-ish.
                    // Ideally, we would need to handle arbitrary
                    // lengths of requests.
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = Self::handle_request(&mut buffer);
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send respose to client: {}", e);
                            }
                        },
                        Err(e) => {
                            println!("Failed to read from stream due to error: {}", e)
                        },
                    }
                },
                Err(e) => {
                    println!("Failed to establish connection due to error: {}", e)
                },
            }
        }
    }
}
