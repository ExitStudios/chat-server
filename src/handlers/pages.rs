use std::fs;

use crate::{
    server::{http_request::HttpRequest, http_response::HttpResponse, router::AppContext},
    utils::consts::paths,
};

pub fn not_found_handler() -> HttpResponse {
    HttpResponse::not_found()
}

pub fn base_handler(_request: HttpRequest, _state: &AppContext) -> HttpResponse {
    HttpResponse::html(fs::read(paths::base_html()).unwrap())
}

pub fn test_post_handler(_erquest: HttpRequest, _state: &AppContext) -> HttpResponse {
    HttpResponse::html(fs::read(paths::base_html()).unwrap())
}
