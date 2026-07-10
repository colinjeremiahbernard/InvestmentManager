use axum::Router;
use sqlx::PgPool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

pub fn create_app(
    pool: PgPool,
    jwt_secret: String,
) -> Router {
       Router::new()
         .merge(routes::router())
         .with_state(AppState { 
            pool,
            jwt_secret,
         })
}