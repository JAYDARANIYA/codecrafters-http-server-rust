use crate::constants::{CRLF, DOUBLE_CRLF};

use super::http_status::HttpStatus;

pub struct HttpResponse {
    pub status: HttpStatus,
}

impl HttpResponse {
    pub fn from_status(status: HttpStatus) -> HttpResponse {
        HttpResponse { status: status }
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();

        response.push_str(&format!(
            "HTTP/1.1 {} {}\r\n\r\n",
            self.status.get_status(),
            self.status.get_message()
        ));

        response
    }
}
