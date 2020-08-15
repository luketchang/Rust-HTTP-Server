use super::server::Handler;
use super::http::{ Request, Response, Method, StatusCode };
use std::fs;

/* Struct: SiteHandler
 * ___________________
 *  - handler object which contains a public site path
 */
pub struct SiteHandler {
    public_path: String
}

/* Implementation: SiteHandler
 * ___________________________
 *  - Function: new
 *      - takes in new public path and returns new SiteHandler object
 *  - Function: read_file
 *      - first creates path by appending client generated file path to public path
 *      - then removes all ..s (for security) and checks that path starts with public path
 *      - returns full file path if safe and None otherwise
 */
impl SiteHandler {
    pub fn new(public_path: String) -> Self {
        SiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String>{
        let path = format!("{}/{}", &self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack");
                    None
                }
            }
            Err(_) => None
        }
        
    }
}

impl Handler for SiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }
}