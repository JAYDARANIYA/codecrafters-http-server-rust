mod http_response;
mod http_status;

use std::{io::Write, net::TcpListener};

use crate::server::{http_response::HttpResponse, http_status::HttpStatus};

pub struct HttpServer;

impl HttpServer {
    pub fn run(&self, addr: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server is running on {}:{}", addr, port);

        let listener = TcpListener::bind(format!("{}:{}", addr, port))?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    println!("New connection: {}", _stream.peer_addr()?);
                    let response = HttpResponse::from_status(HttpStatus::Ok);
                    
                    println!("{}", response.to_string());

                    _stream.write_all(response.to_string().as_bytes())?;
                    _stream.flush()?;
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
