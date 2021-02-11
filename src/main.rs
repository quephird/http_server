struct HttpServer {
    ip_address: String,
    port: String,
}

impl HttpServer {
    fn new(ip_address: String, port: String) -> Self {
        Self {
            ip_address,
            port,
        }
    }

    fn run(self) -> () {
        println!("Running...");
        loop {

        }
    }
}

fn main() {
    let ip_address_and_port = "127.0.0.1:8080";
    let components: Vec<&str> = ip_address_and_port.split(':').collect();
    let ip_address = String::from(components[0]);
    let port = String::from(components[1]);

    println!("Starting new HTTP 1.1 server at address {} and port {}...", ip_address, port);
    let server = HttpServer::new(ip_address, port);
    server.run();
}
