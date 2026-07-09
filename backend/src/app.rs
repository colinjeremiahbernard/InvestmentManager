use axum::Router;
use sqlx::PgPool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn create_app(pool: PgPool) -> Router {
   routes::router()
        .with_state(AppState { pool })
}