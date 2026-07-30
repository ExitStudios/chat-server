use std::{collections::HashMap, fs, io::Write, net::TcpStream};

use crate::utils::consts::paths;

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

    pub fn create_headers(body: &[u8], content_type: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert(
            "Content-Type".to_string(),
            format!("{content_type}; charset=utf-8").to_string(),
        );
        headers.insert("Connection".to_string(), "close".to_string());

        headers
    }

    pub fn html(body: Vec<u8>) -> Self {
        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&body, "text/html"),
            body,
        )
    }

    pub fn css(body: Vec<u8>) -> Self {
        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&body, "text/css"),
            body,
        )
    }

    pub fn json(body: String) -> Self {
        let bytes: Vec<u8> = body.bytes().collect();

        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&bytes, "application/json"),
            bytes,
        )
    }

    pub fn js(body: Vec<u8>) -> Self {
        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&body, "application/js"),
            body,
        )
    }

    pub fn text(body: Vec<u8>) -> Self {
        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&body, "text/plain"),
            body,
        )
    }

    pub fn file(_path: &str) -> Self {
        // HttpResponse::new(
        //     StatusCode::OK,
        //     Self::create_headers(body.clone(), "text/html"),
        //     body,
        // )

        todo!()
    }

    pub fn not_found() -> Self {
        let body = fs::read(paths::not_found_html()).unwrap();

        HttpResponse::new(
            StatusCode::NotFound,
            Self::create_headers(&body, "text/html"),
            body,
        )
    }
}
