use std::collections::HashMap;

use super::http_status::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn from_status(status: HttpStatus) -> HttpResponse {
        HttpResponse {
            status,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn new(status: HttpStatus, headers: HashMap<String, String>, body: String) -> HttpResponse {
        HttpResponse {
            status,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();

        response.push_str(&format!("HTTP/1.1 {}\r\n", self.status));

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        if !self.body.is_empty() {
            response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        }

        response.push_str("\r\n");

        if !self.body.is_empty() {
            response.push_str(&self.body);
        }

        response
    }
}
