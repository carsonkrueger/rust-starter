use tokio::net::TcpListener;

use crate::{context::AppContext, env::config::Config, services::ServiceManager};

mod app_router;
mod context;
mod env;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let cfg = Config::parse();

    let svc = ServiceManager::default();
    let ctx = AppContext { cfg, svc };
    let router = app_router::build_router(ctx.clone());

    let addr = format!("0.0.0.0:{}", ctx.cfg.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on: {}", &addr));

    println!("Listening on: {}", &addr);

    axum::serve(listener, router)
        .await
        .expect("could not serve application")
}
