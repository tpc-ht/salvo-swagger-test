use router::init_router;
use salvo::prelude::*;

mod controller;
mod entity;
mod model;
mod router;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let router = init_router();
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
