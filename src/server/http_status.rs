pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    InternalServerError = 500,
}

impl HttpStatus {
    pub fn get_status(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::NotFound => 404,
            HttpStatus::InternalServerError => 500,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}
