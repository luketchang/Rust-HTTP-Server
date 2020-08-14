use std::io::Read;
use std::net::TcpListener;

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
 *      - takes in self parameter
 *      - listens for tcp binding
 *      - runs loop that establishes connection and listens for requests
 *      - once request is received and written into buffer successfully, print request text
 */
impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address: address
        }
    }

    pub fn run(self) {
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
                        }
                        Err(e) => println!("Failed to read connection: {}", e),
                    }
                }

                Err(e) => println!("Failed to establish connection: {}", e),
            }
        };
    }
}