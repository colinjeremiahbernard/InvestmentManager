use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    auth::extractor::AuthenticatedUser,
    repository::portfolio_repository::PortfolioRepository,
};

pub async fn summary(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("REPORT HANDLER CALLED");
    let repository = PortfolioRepository::new(state.pool.clone());

    let portfolios = match repository.list_by_user(user_id).await {
        Ok(p) => p,
        Err(_) => return Json(serde_json::json!({
            "total_invested":0.0,
            "total_value":0.0,
            "total_gain_loss":0.0,
            "total_gain_loss_pct":0.0,
            "portfolios":[]
        })),
    };

    let mut total_invested = 0.0;
    let mut total_value = 0.0;

    let mut chart = Vec::new();

    for portfolio in portfolios {

        let items = repository
            .list_items(portfolio.id)
            .await
            .unwrap_or_default();

        let invested: f64 =
            items.iter().map(|i| i.total_invested).sum();

        let value: f64 =
            items.iter().map(|i| i.current_value).sum();

        total_invested += invested;
        total_value += value;

        chart.push(serde_json::json!({
            "name": portfolio.name,
            "value": value
        }));
    }

    let gain = total_value - total_invested;

    let gain_pct = if total_invested == 0.0 {
        0.0
    } else {
        gain / total_invested * 100.0
    };

    Json(serde_json::json!({

        "total_invested": total_invested,

        "total_value": total_value,

        "total_gain_loss": gain,

        "total_gain_loss_pct": gain_pct,

        "portfolios": chart

    }))
}