use std::{collections::HashMap, fs};

use crate::server::{
    http_request::HttpRequest,
    http_response::{HttpResponse, StatusCode},
};

pub mod api;
pub mod pages;
pub mod static_files;

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

pub fn create_handler(
    _request: HttpRequest,
    file_path: &str,
    content_type: &str,
    code: StatusCode,
) -> HttpResponse {
    let body = fs::read(file_path).unwrap();

    HttpResponse::new(code, create_headers(&body, content_type), body)
}
