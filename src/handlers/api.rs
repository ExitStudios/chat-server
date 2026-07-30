use std::collections::HashMap;

use serde::Serialize;

use crate::{
    models::message::Message,
    server::{
        http_request::HttpRequest,
        http_response::{HttpResponse, StatusCode},
        router::AppContext,
    },
};

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

pub fn test_post() -> HttpResponse {
    let response_body = b"POST received successfully".to_vec();

    let mut headers = HashMap::new();

    headers.insert(
        "Content-Length".to_string(),
        response_body.len().to_string(),
    );

    headers.insert("Content-Type".to_string(), "text/plain".to_string());

    HttpResponse::new(StatusCode::OK, headers, response_body)
}

pub fn post_message(request: HttpRequest, state: &AppContext) -> HttpResponse {
    let message: Message = match serde_json::from_slice(&request.body) {
        Ok(msg) => msg,
        Err(_) => return HttpResponse::bad_request(),
    };

    let mut state = state.lock().unwrap();
    state.messages.push(message);

    HttpResponse::json(StatusCode::Created, SuccessResponse { success: true })
}

pub fn get_messages(_request: HttpRequest, state: &AppContext) -> HttpResponse {
    let state = state.lock().unwrap();

    HttpResponse::json(StatusCode::OK, &state.messages)
}
