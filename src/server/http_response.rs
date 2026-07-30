use std::{collections::HashMap, io::Write, net::TcpStream};

use serde::Serialize;

#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn to_http_string(&self) -> String {
        match self {
            StatusCode::OK => "HTTP/1.1 200 OK".to_string(),
            StatusCode::Created => "HTTP/1.1 201 CREATED".to_string(),
            StatusCode::BadRequest => "HTTP/1.1 400 BAD REQUEST".to_string(),
            StatusCode::NotFound => "HTTP/1.1 404 NOT FOUND".to_string(),
            StatusCode::InternalServerError => "HTTP/1.1 500 INTERNAL SERVER ERROR".to_string(),
        }
    }

    pub fn to_status_code(status: &str) -> StatusCode {
        match status {
            "HTTP/1.1 200 OK" => StatusCode::OK,
            "HTTP/1.1 201 CREATED" => StatusCode::Created,
            "HTTP/1.1 400 BAD REQUEST" => StatusCode::BadRequest,
            "HTTP/1.1 404 NOT FOUND" => StatusCode::NotFound,
            "HTTP/1.1 500 INTERNAL SERVER ERROR" => StatusCode::InternalServerError,
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

    pub fn json<T: Serialize>(code: StatusCode, data: T) -> Self {
        let data = serde_json::to_vec(&data).unwrap();

        HttpResponse::new(code, Self::create_headers(&data, "application/json"), data)
    }

    pub fn js(body: Vec<u8>) -> Self {
        HttpResponse::new(
            StatusCode::OK,
            Self::create_headers(&body, "application/js"),
            body,
        )
    }

    pub fn text(code: StatusCode, text: &str) -> Self {
        let bytes: Vec<u8> = text.bytes().collect();

        HttpResponse::new(code, Self::create_headers(&bytes, "text/plain"), bytes)
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
        HttpResponse::text(StatusCode::NotFound, "404: Not found")
    }

    pub fn bad_request() -> HttpResponse {
        HttpResponse::text(StatusCode::BadRequest, "Invalid request")
    }
}
