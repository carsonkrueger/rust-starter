use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::env::config::Config;

mod env;

#[tokio::main]
async fn main() {
    let cfg = Config::parse();
    let router = Router::new().route("/", get(|| async { "Hello World" }));

    let addr = format!("0.0.0.0:{}", cfg.port);
    println!("Listening on: {}", &addr);

    let listener = TcpListener::bind(addr).await.expect("invalid listener");
    axum::serve(listener, router)
        .await
        .expect("could not serve application")
}
