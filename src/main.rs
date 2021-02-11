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
        println!("Listening on port {}...", self.port);
        loop {

        }
    }
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
    headers: Vec<String>,
    body: String,
}

fn main() {
    let ip_address_and_port = "127.0.0.1:8080";
    let components: Vec<&str> = ip_address_and_port.split(':').collect();
    let ip_address = String::from(components[0]);
    let port = String::from(components[1]);

    println!("Starting new HTTP 1.1 server...");
    let server = HttpServer::new(ip_address, port);
    server.run();
}
