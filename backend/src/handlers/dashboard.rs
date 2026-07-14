use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::{
    app::AppState, auth::extractor::AuthenticatedUser,
    repository::portfolio_repository::PortfolioRepository,
    services::portfolio_service::PortfolioService,
};

#[derive(Serialize)]
pub struct DashboardStats {
    pub total_portfolios: usize,
    pub total_invested: f64,
    pub total_value: f64,
    pub total_gain_loss: f64,
    pub total_gain_loss_pct: f64,
    pub total_investments: usize,
}

pub async fn dashboard(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));

    let portfolios = match service.list(user_id).await {
        Ok(p) => p,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let total_portfolios = portfolios.len();
    let mut total_invested = 0.0f64;
    let mut total_value = 0.0f64;
    let mut total_investments = 0usize;

    for portfolio in &portfolios {
        if let Ok(detail) = service.get_detail(portfolio.id, user_id).await {
            total_invested += detail.total_invested;
            total_value += detail.total_value;
            total_investments += detail.items.len();
        }
    }

    let total_gain_loss = total_value - total_invested;
    let total_gain_loss_pct = if total_invested == 0.0 {
        0.0
    } else {
        (total_gain_loss / total_invested) * 100.0
    };

    let stats = DashboardStats {
        total_portfolios,
        total_invested,
        total_value,
        total_gain_loss,
        total_gain_loss_pct,
        total_investments,
    };

    (StatusCode::OK, Json(stats)).into_response()
}
