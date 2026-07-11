use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub exchange: Option<String>,
    pub currency: String,
    pub current_price: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAssetRequest {
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub exchange: Option<String>,
    pub currency: Option<String>,
    pub current_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub name: String,
    pub asset_type: String,
    pub exchange: Option<String>,
    pub currency: String,
    pub current_price: f64,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct AssetResponse {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub exchange: Option<String>,
    pub currency: String,
    pub current_price: f64,
    pub is_active: bool,
}