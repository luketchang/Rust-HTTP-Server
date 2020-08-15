use std::io::{ Read, Write };
use std::net::TcpListener;
use crate::http::{ Request, Response, StatusCode, ParseError };
use std::convert::TryFrom;

/* Trait: Handler
 * ______________
 *  - handles mapping Request objects into responses
 *  - default bad_request implementation below
 *  - custom handle_request implementation used is in site handler file
 */
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

/* Struct: Server
 * ______________
 *  - server object in which is created on start
 */
pub struct Server {
    address: String
}

/* Implementation: Server
 * ______________________
 *  - Function: new
 *      - creates new instance of server with passed in String address
 *  - Function: run
 *      - takes in self parameter and mutable reference to handler
 *      - listens for tcp binding
 *      - runs loop that establishes connection and listens for requests
 *      - once request is received and written into buffer successfully, convert it to Request type
 *      - then the Request is mapped to a response using the Handler trait, which is written to the tcp stream 
 *        (or if error writing to stream, then error printed)
 */
impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address: address
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on: {}", self.address);

        //bind listener to address and terminate if binding failed
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //max buffer size is 1024 bytes
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                }

                Err(e) => println!("Failed to establish connection: {}", e),
            }
        };
    }
}