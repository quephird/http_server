use std::io::Read;
use std::net::{SocketAddrV4, TcpListener};


pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
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
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
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
