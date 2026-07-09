use axum::Router;
use crate::app::AppState;

pub mod api;
pub mod frontend;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(api::router())
        .merge(frontend::router())
}