use color_eyre::Result;
use tokio::net::TcpListener;

mod app;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let app = app::create_app();

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("🚀 Investment Manager running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}