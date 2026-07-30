use std::fs;

use crate::{
    server::{http_request::HttpRequest, http_response::HttpResponse, router::AppContext},
    utils::consts::paths,
};

pub fn style_handler(_request: HttpRequest, _state: &AppContext) -> HttpResponse {
    HttpResponse::css(fs::read(paths::base_style()).unwrap())
}

pub fn script_handler(_request: HttpRequest, _state: &AppContext) -> HttpResponse {
    HttpResponse::js(fs::read(paths::base_script()).unwrap())
}

pub fn image_handler() -> HttpResponse {
    todo!()
}

pub fn font_handler() -> HttpResponse {
    todo!()
}

pub fn favicon_handler() -> HttpResponse {
    todo!()
}
