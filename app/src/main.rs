use std::{path::Path, time::Duration};

use diesel::r2d2::{ConnectionManager, Pool};

use tokio::net::TcpListener;

use crate::{
    context::AppContext,
    env::{config::Config, db_config::DBConfig},
    repositories::RepositoryManager,
    services::ServiceManager,
};

mod app_router;
mod context;
mod env;
mod middlewares;
pub mod models;
mod repositories;
mod routes;
pub mod schema;
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

    let pool = connection_pool(&db_cfg);

    let repos = RepositoryManager::default();
    let svc = ServiceManager::default(pool, repos);
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

fn connection_pool(db_cfg: &DBConfig) -> repositories::DBPool {
    let manager = ConnectionManager::new(db_cfg.url());
    Pool::builder()
        .test_on_check_out(true)
        .max_size(db_cfg.max_conns)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("Could not build connection pool")
}
