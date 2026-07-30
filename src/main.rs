use std::{
    net::TcpListener,
    println,
    sync::{Arc, Mutex},
};

use crate::server::{Server, router::AppContext, state::ServerState, thread_pool::ThreadPool};

pub mod handlers;
pub mod models;
pub mod server;
pub mod utils;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    let state: AppContext = Arc::new(Mutex::new(ServerState {
        messages: vec![],
        users: vec![],
        next_message_id: 42,
    }));
    let server = Arc::new(Server::new(state));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let server = Arc::clone(&server);

        pool.execute(move || {
            server.handle_connection(stream);
        });
    }

    println!("Shutting down.")
}
