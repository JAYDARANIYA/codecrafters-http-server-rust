use std::collections::HashMap;

use server::{http_response::HttpResponse, http_status::HttpStatus, HttpServer};

mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::default()
        .get("/", |_| HttpResponse::from_status(HttpStatus::Ok))
        .get("/echo/*", |request| HttpResponse {
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: request.path.join("/").replace("/echo/", ""),
        })
        .run("127.0.0.1", 4221)?;
    Ok(())
}
