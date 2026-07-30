use std::fs;

use crate::{
    handlers::create_headers,
    server::{
        http_request::HttpRequest,
        http_response::{HttpResponse, StatusCode},
    },
};

pub fn not_found_handler(_request: HttpRequest) -> HttpResponse {
    let body = fs::read(r"src\public\html\404.html").unwrap();

    HttpResponse::new(StatusCode::OK, create_headers(&body, "text/html"), body)
}

pub fn base_handler(_request: HttpRequest) -> HttpResponse {
    let body = fs::read(r"src\public\html\index.html").unwrap();

    HttpResponse::new(StatusCode::OK, create_headers(&body, "text/html"), body)
}
