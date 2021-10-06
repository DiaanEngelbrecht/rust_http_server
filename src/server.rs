use crate::http::Request;
use std::{convert::TryFrom, io::Read, io::Write, net::TcpListener};
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Established a connection");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                }
                                Err(error) => println!("Failed to parse the request {}", error),
                            }
                        }
                        Err(e) => println!("Failed to read into buffer: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
