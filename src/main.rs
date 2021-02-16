use std::env;

mod http;
use http::server::Server;
use http::handler::HandlerImpl;

fn main() {
    println!("Starting new HTTP 1.1 server...");
    let address = "127.0.0.1:8080";
    let server = Server::new(address.to_string());

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = std::env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path is set to: {}", public_path);
    server.run(&mut HandlerImpl{public_path});
}
