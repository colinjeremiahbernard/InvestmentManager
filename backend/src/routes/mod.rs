use axum::Router;


pub mod api;
pub mod frontend;

pub fn router() -> Router {
    Router::new()
        .merge(api::router())
        .merge(frontend::router())
}