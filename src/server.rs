use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{ParseError, Request, Response, StatusCode, Method};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response {
      match request.method() {
          Method::GET => match request.path() {
            "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
            "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
            _ => Response::new(StatusCode::NotFound, None),
          }
          _ => Response::new(StatusCode::NotFound, None),
        }
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    //Constructor method for struct always called new 
    //All structs have embedded Self type, same as "Server" here
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    //don't want to take ownership so make it a reference
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
          match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }
  }
}