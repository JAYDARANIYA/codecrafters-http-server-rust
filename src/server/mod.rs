mod http_methods;
mod http_request;
pub mod http_response;
pub mod http_status;
mod thread_pool;

use crate::server::{
    http_methods::HttpMethods, http_request::HttpRequest, http_response::HttpResponse,
    http_status::HttpStatus,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

type Handler = Arc<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>;

static GET_ROUTES: Lazy<DashMap<String, Handler>> = Lazy::new(DashMap::new);
static POST_ROUTES: Lazy<DashMap<String, Handler>> = Lazy::new(DashMap::new);

pub struct HttpServer;

impl Default for HttpServer {
    fn default() -> Self {
        HttpServer {}
    }
}

impl HttpServer {
    pub fn get<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(&HttpRequest) -> HttpResponse + 'static + Send + Sync,
    {
        GET_ROUTES.insert(path.to_string(), Arc::new(handler));
        self
    }

    pub fn post<F>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(&HttpRequest) -> HttpResponse + 'static + Send + Sync,
    {
        POST_ROUTES.insert(path.to_string(), Arc::new(handler));
        self
    }

    pub fn run(&self, addr: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server is running on {}:{}", addr, port);

        let listener = TcpListener::bind(format!("{}:{}", addr, port))?;

        let thread_pool = thread_pool::ThreadPool::new(8);

        for stream in listener.incoming() {
            let mut stream = stream?;

            thread_pool.execute(move || {
                if let Err(e) = Self::handle_connection(&mut stream) {
                    println!("Failed to handle connection: {}", e);
                }
            });
        }

        Ok(())
    }

    fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let request = HttpRequest::from_stream(stream)?;

        match request.method {
            HttpMethods::GET => {
                let response = GET_ROUTES
                    .get(&request.path.join("/"))
                    .map(|handler| handler.value()(&request))
                    .or_else(|| {
                        GET_ROUTES.iter().find_map(|entry| {
                            let path = entry.key();
                            if path.contains("/*")
                                && request.path.join("/").starts_with(&path.replace("/*", ""))
                            {
                                Some(entry.value()(&request))
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or_else(|| HttpResponse::from_status(HttpStatus::NotFound));

                stream.write_all(response.to_string().as_bytes())?;
                stream.flush()?;
            }
            HttpMethods::POST => {
                let response = POST_ROUTES
                    .get(&request.path.join("/"))
                    .map(|handler| handler.value()(&request))
                    .or_else(|| {
                        POST_ROUTES.iter().find_map(|entry| {
                            let path = entry.key();
                            if path.contains("/*")
                                && request.path.join("/").starts_with(&path.replace("/*", ""))
                            {
                                Some(entry.value()(&request))
                            } else {
                                None
                            }
                        })
                    })
                    .unwrap_or_else(|| HttpResponse::from_status(HttpStatus::NotFound));

                stream.write_all(response.to_string().as_bytes())?;
                stream.flush()?;
            }
            _ => {
                let response = HttpResponse::from_status(HttpStatus::NotFound);
                stream.write_all(response.to_string().as_bytes())?;
                stream.flush()?;
            }
        }

        Ok(())
    }
}
