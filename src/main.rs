#![allow(dead_code)]
use http::Method;
use http::Request;
use server::Server;
use site_handler::SiteHandler;

mod http;
mod server;
mod site_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(SiteHandler);
}