use server::{http_response::HttpResponse, http_status::HttpStatus, HttpServer};

mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::default()
        .get("/", || HttpResponse::from_status(HttpStatus::Ok))
        .run("127.0.0.1", 4221)?;
    Ok(())
}
