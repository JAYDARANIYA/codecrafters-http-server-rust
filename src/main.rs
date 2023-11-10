use std::collections::HashMap;

use server::{http_response::HttpResponse, http_status::HttpStatus, HttpServer};

mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read --directory directory/  argument
    let args: Vec<String> = std::env::args().collect();

    let directory = match args.get(2).cloned() {
        Some(directory) => directory,
        None => {
            // return current directory
            std::env::current_dir()?.to_string_lossy().to_string()
        }
    };

    println!("Serving directory: {}", directory);

    HttpServer::default()
        .get("/", |_| HttpResponse::from_status(HttpStatus::Ok))
        .get("/echo/*", |request| HttpResponse {
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: request.path.join("/").replace("/echo/", ""),
        })
        .get("/user-agent", |request| HttpResponse {
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: match request.headers.get("User-Agent") {
                Some(user_agent) => user_agent.to_string(),
                None => "No User-Agent header".to_string(),
            },
        })
        .get("/file/*", move |request| {
            let path = request.path.join("/").replace("/files/", "");
            let file_path = format!("{}/{}", directory.clone(), path);
            match std::fs::read_to_string(file_path) {
                Ok(content) => HttpResponse {
                    status: HttpStatus::Ok,
                    headers: {
                        let mut headers = HashMap::new();
                        headers.insert(
                            "Content-Type".to_string(),
                            "application/octet-stream".to_string(),
                        );
                        headers
                    },
                    body: content,
                },
                Err(_) => HttpResponse::from_status(HttpStatus::NotFound),
            }
        })
        .run("127.0.0.1", 4221)?;
    Ok(())
}
