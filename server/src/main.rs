#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let server = Server::new(addr);
    server.run(WebsiteHandler)
}
