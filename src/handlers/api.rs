use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    models::message::Message,
    server::{
        http_request::HttpRequest,
        http_response::{HttpResponse, StatusCode},
        state::ServerState,
    },
};

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

pub fn post_message(request: HttpRequest, state: Arc<Mutex<ServerState>>) -> HttpResponse {
    let message: Message = serde_json::from_slice(&request.body).unwrap();

    println!("msg: {:#?}", message);

    let mut state = state.lock().unwrap();
    state.messages.push(message);

    HttpResponse::json(r#"{"success":true}"#.to_string())
}

pub fn get_messages(_request: HttpRequest, state: Arc<Mutex<ServerState>>) -> HttpResponse {
    let state = state.lock().unwrap();
    let json = serde_json::to_string(&state.messages).unwrap();

    HttpResponse::json(json)
}
