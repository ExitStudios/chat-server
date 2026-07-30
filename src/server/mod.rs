use std::{
    io::{BufReader, Write},
    net::TcpStream,
};

use crate::{
    handlers::api,
    server::{
        http_request::HttpRequest,
        http_response::HttpResponse,
        router::{AppContext, Router},
    },
};

pub mod http_request;
pub mod http_response;
pub mod router;
pub mod state;
pub mod thread_pool;

pub struct Server {
    pub router: Router,
    state: AppContext,
}

impl Server {
    pub fn new(state: AppContext) -> Self {
        let mut router = Router::new();

        Self::setup_routes(&mut router);

        Self { router, state }
    }

    fn setup_routes(router: &mut Router) {
        router.get("/api/messages", api::get_messages);
        router.post("/api/messages", api::post_message);
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut reader = BufReader::new(&stream);

        let request_string = HttpRequest::stringify_stream(&mut reader);
        let request = match HttpRequest::from_request_string(request_string) {
            Ok(req) => req,
            Err(_) => {
                let response = HttpResponse::bad_request();
                response.send(&mut stream);
                return;
            }
        };

        println!("{} {}", request.method, request.path);

        let response = self.router.handle(request, &self.state);
        response.send(&mut stream);

        stream.flush().unwrap();
    }
}
