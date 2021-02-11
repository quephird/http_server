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
            let res = listener.accept();

            match res {
                Err(e) => {
                    println!("Failed to establish connection due to error: {}", e)
                },
                Ok((stream, address)) => {
                    println!("Received connection from {}!!!", address);
                }
            }
        }
    }
}
