use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Uuid,
    pub transaction_type: String,
    pub quantity: f64,
    pub price_per_unit: f64,
    pub total_amount: f64,
    pub notes: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Uuid,
    pub asset_symbol: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub price_per_unit: f64,
    pub total_amount: f64,
    pub notes: Option<String>,
    pub transaction_date: DateTime<Utc>,
}
