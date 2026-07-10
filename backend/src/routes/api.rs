use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app::AppState,
    handlers::{auth, me},
};

async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/me", get(me::me))
}