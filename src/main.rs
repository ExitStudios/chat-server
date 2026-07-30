use std::{net::TcpListener, println};

use crate::server::thread_pool::ThreadPool;

pub mod handlers;
pub mod server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            server::handle_connection(stream);
        });
    }

    println!("Shutting down.")
}
