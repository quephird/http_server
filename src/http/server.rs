pub struct Server {
    ip_address: String,
    port: String,
}

impl Server {
    pub fn new(ip_address: String, port: String) -> Self {
        Self {
            ip_address,
            port,
        }
    }

    pub fn run(self) -> () {
        println!("Listening on port {}...", self.port);
        loop {

        }
    }
}
