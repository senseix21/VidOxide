use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/healthz", get(|| async { "ok" }))
}