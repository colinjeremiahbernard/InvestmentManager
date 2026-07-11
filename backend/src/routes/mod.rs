use axum::Router;
use tower_http::services::ServeDir;
use crate::app::AppState;

pub mod api;
pub mod frontend;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(api::router())
        .merge(frontend::router())
        .nest_service("/static", ServeDir::new("static"))
}