use std::sync::{Arc, Mutex};

use crate::{
    handlers::{
        pages::{base_handler, not_found_handler},
        static_files::{script_handler, style_handler},
    },
    server::{
        http_request::{HttpMethod, HttpRequest},
        http_response::HttpResponse,
        state::ServerState,
    },
};

type Handler = fn(HttpRequest, Arc<Mutex<ServerState>>) -> HttpResponse;

pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: Handler,
}

impl Route {
    pub fn new(method: HttpMethod, path: &str, handler: Handler) -> Self {
        Self {
            method,
            path: path.to_string(),
            handler,
        }
    }

    pub fn matches(&self, request: &HttpRequest) -> bool {
        // self.method == request.method && self.path == request.path
        self.path == request.path
    }
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        let base_route = Route::new(HttpMethod::GET, "/", base_handler);
        let style_route = Route::new(HttpMethod::GET, "/style.css", style_handler);
        let script_route = Route::new(HttpMethod::GET, "/script.js", script_handler);

        Self {
            routes: vec![base_route, style_route, script_route],
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(Route::new(HttpMethod::GET, path, handler));
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(Route::new(HttpMethod::POST, path, handler));
    }

    pub fn find_route(&self, request: &HttpRequest) -> Option<&Route> {
        self.routes.iter().find(|route| route.matches(request))
    }

    pub fn handle(&self, request: HttpRequest, state: Arc<Mutex<ServerState>>) -> HttpResponse {
        match self.find_route(&request) {
            Some(route) => (route.handler)(request, state),
            None => {
                println!("Route not found: {:#?}", request.path);
                not_found_handler()
            }
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
