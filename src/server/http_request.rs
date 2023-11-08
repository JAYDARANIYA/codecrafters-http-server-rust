use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use super::http_methods::HttpMethods;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethods,
    pub path: Vec<String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn from_stream(stream: &mut TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(stream);

        let mut request = HttpRequest {
            method: HttpMethods::GET,
            path: Vec::new(),
            version: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        };

        let mut line = String::new();

        reader.read_line(&mut line)?;

        let mut parts = line.split_whitespace();

        request.method = HttpMethods::from_string(parts.next().unwrap());

        request.path = parts
            .next()
            .unwrap()
            .split("/")
            .map(|s| s.to_string())
            .collect();

        request.version = parts.next().unwrap().to_string();

        let mut line = String::new();

        while reader.read_line(&mut line)? > 2 {
            let mut parts = line.split(": ");

            request.headers.insert(
                parts.next().unwrap().trim().to_string(),
                parts.next().unwrap().trim().to_string(),
            );

            line = String::new();
        }

        if request.headers.contains_key("Content-Length") {
            let content_length = request
                .headers
                .get("Content-Length")
                .unwrap()
                .parse::<usize>()?;

            let mut buffer = vec![0; content_length];

            reader.read_exact(&mut buffer)?;

            request.body = String::from_utf8(buffer)?;
        } else {
            request.body = String::new();
        }

        Ok(request)
    }
}
