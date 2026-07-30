use std::{collections::HashMap, io::Write, net::TcpStream};

#[derive(Debug)]
pub enum StatusCode {
    OK,
    NotFound,
}

impl StatusCode {
    pub fn to_http_string(&self) -> &str {
        match self {
            StatusCode::OK => "HTTP/1.1 200 OK",
            StatusCode::NotFound => "HTTP/1.1 404 NOT FOUND",
        }
    }

    pub fn to_status_code(status: &str) -> StatusCode {
        match status {
            "HTTP/1.1 200 OK" => StatusCode::OK,
            _ => StatusCode::NotFound,
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: StatusCode, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
        let header = self.get_string_header();

        stream.write_all(header.as_bytes()).unwrap();
        stream.write_all(&self.body).unwrap();
    }

    pub fn get_string_header(&self) -> String {
        let mut response_header = format!("{}\r\n", StatusCode::to_http_string(&self.status));

        for (key, value) in &self.headers {
            response_header.push_str(&format!("{key}: {value}\r\n"));
        }

        response_header.push_str("\r\n");

        response_header
    }
}
