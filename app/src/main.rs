use std::{path::Path, time::Duration};

use bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tokio::net::TcpListener;
use tracing::error;
use tracing_subscriber::EnvFilter;
use utils::auth::{privileges::Privilege, roles::ROLE_ADMIN};

use crate::{
    context::AppState,
    env::{config::Config, db_config::DBConfig},
    repositories::{DbConn, RepositoryManager},
    services::{
        ServiceManager,
        privileges::{Privileges, PrivilegesService},
    },
};

use strum::IntoEnumIterator;

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
    let svc = ServiceManager::<_, Privileges, _>::default(pool, repos);
    let priv_service = svc.privileges.clone();
    let ctx = AppState {
        cfg: cfg.clone(),
        svc,
    };

    let filter = EnvFilter::new("")
        .add_directive("app=trace".parse().unwrap())
        .add_directive("utils=warn".parse().unwrap())
        .add_directive("templates=error".parse().unwrap())
        .add_directive("models=error".parse().unwrap())
        .add_directive("schemas=error".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let privs: Vec<Privilege> = Privilege::iter().collect();
    if let Err(e) = priv_service.associate(ROLE_ADMIN, &privs).await {
        error!("Failed to associate admin privileges: {}", e);
        panic!("Failed to associate admin privileges: {}", e);
    }
    let router = routes::build_router(ctx.clone());

    let addr = format!("localhost:{}", ctx.cfg.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on: {}", &addr));

    println!("Listening on: http://{}", addr);

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
