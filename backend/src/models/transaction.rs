use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionType {
    BUY,
    SELL,
    DIVIDEND,
    DEPOSIT,
    WITHDRAWAL,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub transaction_type: String,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub portfolio_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub transaction_type: String,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub amount: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionRequest {
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub amount: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub transaction_type: String,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(t: Transaction) -> Self {
        Self {
            id: t.id,
            portfolio_id: t.portfolio_id,
            asset_id: t.asset_id,
            transaction_type: t.transaction_type,
            quantity: t.quantity,
            price: t.price,
            amount: t.amount,
            notes: t.notes,
            created_at: t.created_at,
        }
    }
}
