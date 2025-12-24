use std::{path::Path, time::Duration};

use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tokio::net::TcpListener;

use crate::{
    context::AppState,
    env::{config::Config, db_config::DBConfig},
    repositories::{DbConn, RepositoryManager},
    services::ServiceManager,
};

mod app_templates;
mod context;
mod env;
mod middlewares;
mod repositories;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let env_path = match args.get(1) {
        Some(arg) => Some(Path::new(arg)),
        None => None,
    };

    let cfg = Config::parse(env_path);
    let db_cfg = DBConfig::parse();

    // let pool = connection_pool(&db_cfg);
    let pool = connection_pool(&db_cfg).await;

    let repos = RepositoryManager::default();
    let svc = ServiceManager::default(pool, repos);
    let ctx = AppState { cfg, svc };
    let router = routes::build_router(ctx.clone());

    let addr = format!("localhost:{}", ctx.cfg.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on: {}", &addr));

    println!("Listening on: {}", format!("http://{}", addr));

    axum::serve(listener, router)
        .await
        .expect("could not serve application")
}

async fn connection_pool(db_cfg: &DBConfig) -> repositories::DBPool {
    let manager = AsyncDieselConnectionManager::<DbConn>::new(db_cfg.url());
    Pool::builder()
        .test_on_check_out(true)
        .max_size(db_cfg.max_conns)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .await
        .expect("Could not build connection pool")
}
