use std::{
    net::TcpListener,
    println,
    sync::{Arc, Mutex},
};

use crate::server::{state::ServerState, thread_pool::ThreadPool};

pub mod handlers;
pub mod models;
pub mod server;
pub mod utils;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    let state = Arc::new(Mutex::new(ServerState { messages: vec![] }));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let state = Arc::clone(&state);

        pool.execute(move || {
            server::handle_connection(stream, state);
        });
    }

    println!("Shutting down.")
}
