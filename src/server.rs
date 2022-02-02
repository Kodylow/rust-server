use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

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
    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
          match listener.accept() {
            Ok((mut stream, _)) => {
              let mut buffer = [0; 1024];
              match stream.read(&mut buffer) {
                Ok(_) => {
                  println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                  match Request::try_from(&buffer[..]) {
                    Ok(request) => {},
                    Err(e) => println!("Failed to parse a request: {}", e)
                  }
                  // let res: &Result<Request, _> = &buffer[..].try_into();
                },
                Err(e) => println!("Failed to read from connection: {}", e),
              }
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
          }
        }
    }
}