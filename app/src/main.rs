use diesel::r2d2::{ConnectionManager, Pool};

use tokio::net::TcpListener;

use crate::{
    context::AppContext,
    env::{config::Config, db_config::DBConfig},
    services::ServiceManager,
};

mod app_router;
mod context;
mod env;
mod repositories;
mod routes;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let cfg = Config::parse();
    let db_cfg = DBConfig::parse();

    // let pg_conn = PgConnection::establish(&db_cfg.url()).expect("Could not connect to database");
    let pool = connection_pool(&db_cfg);

    let svc = ServiceManager::default(pool);
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
        .build(manager)
        .expect("Could not build connection pool")
}
