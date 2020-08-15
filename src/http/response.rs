use super::status_code::StatusCode;
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };
use std::io::{ Write, Result as IoResult };
use std::net::TcpStream;

/* Struct: Response
 * ________________
 *  - Response object containing status code and response body
 */
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

/* Implementation: Response
 * ________________________
 *  - Function: new
 *      - takes in status code and body and returns new Response object
 *  - Function: send
 *      - takes in reference to self and TcpStream, matching a string from the body field,
 *        and writing the response in the correct format back to the stream (HTTP/1.1 status_code message body)
 */
impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            &self.status_code, 
            &self.status_code.message(),
            body
        )
    }
}