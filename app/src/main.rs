use std::{path::Path, sync::Arc, time::Duration};

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
    // Parse env
    let args = std::env::args().collect::<Vec<String>>();
    let env_path = match args.get(2) {
        Some(arg) => Some(Path::new(arg)),
        None => None,
    };
    let cfg = Config::parse(env_path);
    let db_cfg = DBConfig::parse(cfg.internal);

    // Setup app state: db connections, repositories and services
    let pool = connection_pool(&db_cfg).await;
    let repos = RepositoryManager::default();
    let svc = ServiceManager::default(pool.clone(), repos);
    let ctx = AppState {
        cfg: cfg.clone(),
        svc,
    };

    // Setup tracing
    let filter = EnvFilter::new("")
        .add_directive("app=trace".parse().unwrap())
        .add_directive("utils=warn".parse().unwrap())
        .add_directive("templates=error".parse().unwrap())
        .add_directive("models=error".parse().unwrap())
        .add_directive("schemas=error".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Associate all privileges to the admin role
    let temp_priv_service = Privileges::new(pool, Arc::new(RepositoryManager::default()));
    let privs: Vec<Privilege> = Privilege::iter().collect();
    if let Err(e) = temp_priv_service.associate(ROLE_ADMIN, &privs).await {
        error!("Failed to associate admin privileges: {}", e);
        panic!("Failed to associate admin privileges: {}", e);
    }

    // Build router
    let router = routes::build_router(ctx.clone());
    let addr = format!("{}:{}", ctx.cfg.host, ctx.cfg.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on: {}", &addr));

    println!("Listening on: http://{}", addr);

    // Start server
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
