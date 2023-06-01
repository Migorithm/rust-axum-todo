mod db;
mod handlers;
mod models;
mod schema;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, patch},
    Router,
};
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::{PostgresStore, Store, StoreType};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Arc::new(RwLock::new(
        db::PostgresStore::new("postgres://migo:abc123@localhost:5433/rust-todo")
            .await
            .expect("This Must Not Be Failed!"),
    ));
    println!("Db Connected!");

    // Compose the routes
    let app = Router::new()
        .route(
            "/todos",
            get(handlers::todos_index).post(handlers::todos_create),
        )
        // .route(
        //     "/todos/:id",
        //     patch(handlers::todos_update).delete(handlers::todos_delete),
        // )
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db.clone());

    tracing::debug!("listening...");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// The query parameters for todos index
