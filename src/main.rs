use anyhow::Context;
use axum::{http::StatusCode, Json};
use config::Config;
use connection_pool::ConnectionManager;
use dotenv::dotenv;
use service_register::ServiceRegister;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::router::AppRouter;

mod abstract_trait;
mod config;
mod connection_pool;
mod errors;
mod handler;
mod models;
mod repository;
mod request;
mod response;
mod router;
mod service;
mod service_register;

pub type JsonResponse<T> = (StatusCode, Json<T>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::init();

    init_tracing();

    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let port = config.port;
    let service_register = ServiceRegister::new(pg_pool);

    println!("🚀 Server started successfully");

    AppRouter::serve(port, service_register)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=debug,axum_note=debug").unwrap_or_else(|_| {
                if cfg!(test) {
                    "tower_http=error"
                } else {
                    "axum_note=debug,tower_http=debug"
                }
                .into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
