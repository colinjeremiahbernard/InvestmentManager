use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use tera::Context;
use uuid::Uuid;
use crate::app::AppState;

async fn index(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("index.html", &Context::new())
        .expect("Failed to render index.html");

    Html(html)
}

async fn login(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("login.html", &Context::new())
        .expect("Failed to render login.html");

    Html(html)
}

async fn register(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("register.html", &Context::new())
        .expect("Failed to render register.html");

    Html(html)
}

async fn dashboard(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("dashboard.html", &Context::new())
        .expect("Failed to render dashboard.html");

    Html(html)
}
async fn assets(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("assets.html", &Context::new())
        .expect("Failed to render assets.html");

    Html(html)
}

async fn portfolios(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("portfolios.html", &Context::new())
        .expect("Failed to render portfolios.html");

    Html(html)
}

async fn transactions(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("transactions.html", &Context::new())
        .expect("Failed to render transactions.html");

    Html(html)
}

async fn reports(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("reports.html", &Context::new())
        .expect("Failed to render reports.html");

    Html(html)
}

async fn settings(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render("settings.html", &Context::new())
        .expect("Failed to render settings.html");

    Html(html)
}
async fn portfolio_detail(
    State(state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> impl IntoResponse {

    let html = state
        .tera
        .render(
            "portfolio_detail.html",
            &tera::Context::new()
        )
        .unwrap();

    Html(html)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/register", get(register))
        .route("/dashboard", get(dashboard))
        .route("/assets", get(assets))
        .route("/portfolios", get(portfolios))
        .route("/transactions", get(transactions))
        .route("/reports", get(reports))
        .route("/settings", get(settings))
        .route("/portfolio/{id}",
         get(portfolio_detail))
        
}