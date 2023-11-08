#[derive(Debug, PartialEq)]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

impl HttpMethods {
    pub fn from_string(method: &str) -> HttpMethods {
        match method {
            "GET" => HttpMethods::GET,
            "POST" => HttpMethods::POST,
            "PUT" => HttpMethods::PUT,
            "DELETE" => HttpMethods::DELETE,
            "HEAD" => HttpMethods::HEAD,
            "OPTIONS" => HttpMethods::OPTIONS,
            "CONNECT" => HttpMethods::CONNECT,
            "TRACE" => HttpMethods::TRACE,
            "PATCH" => HttpMethods::PATCH,
            _ => HttpMethods::GET,
        }
    }
}
