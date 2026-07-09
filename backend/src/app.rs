use axum::Router;
use sqlx::PgPool;

use crate::routes;



pub fn create_app(_pool: PgPool) -> Router {
    Router::new()
        .merge(routes::router())
}