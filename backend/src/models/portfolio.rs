use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Portfolio {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePortfolioRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PortfolioResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Portfolio> for PortfolioResponse {
    fn from(p: Portfolio) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            name: p.name,
            description: p.description,
            created_at: p.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PortfolioItem {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Uuid,
    pub quantity: f64,
    pub purchase_price: f64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddPortfolioItemRequest {
    pub asset_id: Uuid,
    pub quantity: f64,
    pub purchase_price: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePortfolioItemRequest {
    pub quantity: f64,
    pub purchase_price: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PortfolioItemResponse {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Uuid,
    pub asset_symbol: String,
    pub asset_name: String,
    pub asset_type: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub current_price: f64,
    pub total_invested: f64,
    pub current_value: f64,
    pub gain_loss: f64,
    pub gain_loss_pct: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PortfolioDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<PortfolioItemResponse>,
    pub total_invested: f64,
    pub total_value: f64,
    pub total_gain_loss: f64,
    pub total_gain_loss_pct: f64,
}
