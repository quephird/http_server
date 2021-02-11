mod http;
use http::server::Server;

fn main() {
    println!("Starting new HTTP 1.1 server...");
    let address = "127.0.0.1:8080";
    let server = Server::new(address.to_string());
    server.run();
}
