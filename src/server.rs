use std::net::TcpListener;

pub struct HttpServer;

impl HttpServer {
    pub fn run(&self, addr: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server is running on {}:{}", addr, port);

        let listener = TcpListener::bind(format!("{}:{}", addr, port))?;

        for stream in listener.incoming() {
            match stream {
                Ok(_stream) => {
                    println!("New connection: {}", _stream.peer_addr()?);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
