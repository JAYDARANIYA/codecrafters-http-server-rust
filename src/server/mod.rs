mod handlers;
mod http_methods;
mod http_request;
pub mod http_response;
pub mod http_status;

use crate::server::{
    http_methods::HttpMethods, http_request::HttpRequest, http_response::HttpResponse,
    http_status::HttpStatus,
};
use std::{collections::HashMap, io::Write, net::TcpListener};

pub struct HttpServer {
    // get routes: HashMap<String, fn(&HttpRequest) -> HttpResponse>,
    get_routes: HashMap<String, Box<dyn Fn(&HttpRequest) -> HttpResponse>>,
}

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {
            // get_routes: HashMap::new(),
            get_routes: HashMap::new(),
        }
    }
}

impl HttpServer {
    pub fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(&HttpRequest) -> HttpResponse + 'static,
    {
        self.get_routes.insert(path.to_string(), Box::new(handler));
        self
    }

    pub fn run(&self, addr: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server is running on {}:{}", addr, port);

        let listener = TcpListener::bind(format!("{}:{}", addr, port))?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut _stream) => {
                    self.handle_connection(&mut _stream)?;
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(
        &self,
        _stream: &mut std::net::TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = HttpRequest::from_stream(_stream)?;

        match request.method {
            HttpMethods::GET => {
                let mut response = HttpResponse::from_status(HttpStatus::NotFound);

                for (path, handler) in &self.get_routes {
                    // if path contains * then we need to check if the request path starts with the path
                    if path.contains("/*") {
                        if request.path.join("/").starts_with(&path.replace("/*", "")) {
                            response = handler(&request);
                        }
                    } else if path == &request.path.join("/") {
                        response = handler(&request);
                    }
                }

                _stream.write_all(response.to_string().as_bytes())?;
                _stream.flush()?;
            }
            _ => {
                let response = HttpResponse::from_status(HttpStatus::NotFound);

                _stream.write_all(response.to_string().as_bytes())?;
                _stream.flush()?;
            }
        }

        Ok(())
    }
}
