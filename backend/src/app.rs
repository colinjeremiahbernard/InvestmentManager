use axum::Router;
use sqlx::PgPool;
use tera::Tera;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
    pub tera: Tera,
}

pub fn create_app(
    pool: PgPool,
    jwt_secret: String,
) -> Router {

    // Load all templates from the templates/ directory
    let tera = Tera::new("templates/**/*")
        .expect("Failed to initialize Tera");

    let state = AppState {
        pool,
        jwt_secret,
        tera,
    };

    Router::new()
        .merge(routes::router())
        .with_state(state)
}