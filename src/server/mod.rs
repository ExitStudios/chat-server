use std::{
    io::{BufReader, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    handlers::api,
    server::{
        http_request::{HttpMethod, HttpRequest},
        router::{Route, Router},
        state::ServerState,
    },
};

pub mod http_request;
pub mod http_response;
pub mod router;
pub mod state;
pub mod thread_pool;

pub fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<ServerState>>) {
    let mut reader = BufReader::new(&stream);
    let request_string = HttpRequest::stringify_stream(&mut reader);
    let request = HttpRequest::from_request_string(request_string);

    println!("{} {}", request.method, request.path);

    let mut router = Router::new();

    router.add_route(Route::new(
        HttpMethod::GET,
        "/api/messages",
        api::get_messages,
    ));
    router.add_route(Route::new(
        HttpMethod::POST,
        "/api/messages",
        api::post_message,
    ));

    let response = router.handle(request, state);
    response.send(&mut stream);

    stream.flush().unwrap();
}
