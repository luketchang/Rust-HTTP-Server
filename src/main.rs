#![allow(dead_code)]
use std::env;
use server::Server;
use site_handler::SiteHandler;

mod http;
mod server;
mod site_handler;

/* Function: main
 * ______________
 *  - gets default path as the project's director with /public appended
 *  - looks for another file path in as user environment variable and defaults to default_path
 *    if not found
 *  - creates new server instance and runs it
 */
fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Path: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(SiteHandler::new(public_path));
}