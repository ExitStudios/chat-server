use std::{
    io::{BufReader, Write},
    net::TcpStream,
};

use crate::server::{http_request::HttpRequest, router::Router};

pub mod http_request;
pub mod http_response;
pub mod router;
pub mod thread_pool;

pub fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let request_string = HttpRequest::stringify_stream(&mut reader);
    let request = HttpRequest::from_request_string(request_string);

    println!("{} {}", request.method, request.path);

    let router = Router::new(); // Creates route for / and /style.css, ...
    let response = router.handle(request);

    response.send(&mut stream);

    stream.flush().unwrap();
}
