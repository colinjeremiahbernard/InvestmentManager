use axum::{
    routing::get,
    Router,
};

async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
}