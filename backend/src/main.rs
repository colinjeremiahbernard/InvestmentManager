use color_eyre::Result;
use tokio::net::TcpListener;

mod app;
mod config;
mod database;
mod routes;
mod handlers;
mod models;
mod services;
mod repository;
mod error;
mod auth;
mod middleware;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let settings = config::settings::Settings::new();

    let pool = database::connection::connect(&settings.database_url).await?;

    println!("✅ Connected to PostgreSQL");

    let app = app::create_app(pool, settings.jwt_secret);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("🚀 Investment Manager running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}