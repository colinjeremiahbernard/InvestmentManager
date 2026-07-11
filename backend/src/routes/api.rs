use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    app::AppState,
    handlers::{asset, auth, dashboard, me, portfolio},
};

async fn health() -> &'static str {
    "OK"
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        // Auth
        .route("/api/register", post(auth::register))
        .route("/api/login", post(auth::login))
        .route("/api/me", get(me::me))
        // Dashboard
        .route("/api/dashboard", get(dashboard::dashboard))
        // Assets
        .route("/api/assets", post(asset::create))
        .route("/api/assets", get(asset::list))
        .route("/api/assets/{id}", get(asset::get))
        .route("/api/assets/{id}", put(asset::update))
        .route("/api/assets/{id}", delete(asset::delete))
        // Portfolios
        .route("/api/portfolios", post(portfolio::create))
        .route("/api/portfolios", get(portfolio::list))
        .route("/api/portfolios/{id}", get(portfolio::get))
        .route("/api/portfolios/{id}", put(portfolio::update))
        .route("/api/portfolios/{id}", delete(portfolio::delete))
        // Portfolio items
        .route("/api/portfolios/{id}/items", post(portfolio::add_item))
        .route("/api/portfolios/{portfolio_id}/items/{item_id}", put(portfolio::update_item))
        .route("/api/portfolios/{portfolio_id}/items/{item_id}", delete(portfolio::remove_item))
}