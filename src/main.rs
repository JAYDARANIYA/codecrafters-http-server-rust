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
        .get("/user-agent", |request| {
            // block thread for 5 seconds
            std::thread::sleep(std::time::Duration::from_secs(15));
            HttpResponse {
                    status: HttpStatus::Ok,
                    headers: HashMap::new(),
                    body: match request.headers.get("User-Agent") {
                        Some(user_agent) => user_agent.to_string(),
                        None => "No User-Agent header".to_string(),
                    },
                }
        })
        .run("127.0.0.1", 4221)?;
    Ok(())
}
