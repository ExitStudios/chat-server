use std::fs;

use crate::{
    handlers::create_headers,
    server::{
        http_request::HttpRequest,
        http_response::{HttpResponse, StatusCode},
    },
};

pub fn style_handler(_request: HttpRequest) -> HttpResponse {
    let body = fs::read(r"src\public\css\index.css").unwrap();

    HttpResponse::new(StatusCode::OK, create_headers(&body, "style/css"), body)
}

pub fn script_handler(_request: HttpRequest) -> HttpResponse {
    let body = fs::read(r"src\public\js\script.js").unwrap();

    HttpResponse::new(
        StatusCode::OK,
        create_headers(&body, "application/js"),
        body,
    )
}
