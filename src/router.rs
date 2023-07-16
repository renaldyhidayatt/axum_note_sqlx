use anyhow::Context;
use axum::{
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde_json::json;
use tracing::info;

use crate::{
    handler::{create_note, delete_note, get_note_id, get_notes, update_note},
    service_register::ServiceRegister,
};

fn routes(service_register: ServiceRegister) -> Router {
    Router::new()
        .route("/notes", get(get_notes))
        .route("/notes/:id", get(get_note_id))
        .route("/notes", post(create_note))
        .route("/notes/:id", put(update_note))
        .route("/notes/:id", delete(delete_note))
        .layer(Extension(service_register.note_service))
}

pub struct AppRouter;

impl AppRouter {
    pub async fn serve(port: u16, service_register: ServiceRegister) -> anyhow::Result<()> {
        let router = Router::new()
            .route("/ping", get(Self::health_checker_handler))
            .nest("/api", routes(service_register));

        info!("routes initialized, listening on port {}", port);
        axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
            .serve(router.into_make_service())
            .await
            .context("error while starting API server")?;

        Ok(())
    }

    pub async fn health_checker_handler() -> impl IntoResponse {
        const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";

        let json_response = json!({
            "status": "success",
            "message": MESSAGE
        });

        Json(json_response)
    }
}
