use crate::models::asset::{Asset, CreateAssetRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub struct AssetRepository {
    pool: PgPool,
}

impl AssetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, request: CreateAssetRequest) -> Result<Asset, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            INSERT INTO assets (symbol, name, asset_type, exchange, currency, current_price)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, symbol, name, asset_type, exchange, currency, current_price, is_active, created_at, updated_at
            "#
        )
        .bind(request.symbol.to_uppercase())
        .bind(request.name)
        .bind(request.asset_type)
        .bind(request.exchange)
        .bind(request.currency.unwrap_or_else(|| "USD".to_string()))
        .bind(request.current_price.unwrap_or(0.0))
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, symbol, name, asset_type, exchange, currency, current_price, is_active, created_at, updated_at
            FROM assets
            WHERE symbol = $1
            "#
        )
        .bind(symbol.to_uppercase())
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, symbol, name, asset_type, exchange, currency, current_price, is_active, created_at, updated_at
            FROM assets
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list(&self) -> Result<Vec<Asset>, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            SELECT id, symbol, name, asset_type, exchange, currency, current_price, is_active, created_at, updated_at
            FROM assets
            ORDER BY symbol
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM assets WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        request: CreateAssetRequest,
    ) -> Result<Asset, sqlx::Error> {
        sqlx::query_as::<_, Asset>(
            r#"
            UPDATE assets
            SET
                symbol       = $2,
                name         = $3,
                asset_type   = $4,
                exchange     = $5,
                currency     = $6,
                current_price = $7,
                updated_at   = NOW()
            WHERE id = $1
            RETURNING id, symbol, name, asset_type, exchange, currency, current_price, is_active, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(request.symbol.to_uppercase())
        .bind(request.name)
        .bind(request.asset_type)
        .bind(request.exchange)
        .bind(request.currency.unwrap_or_else(|| "USD".to_string()))
        .bind(request.current_price.unwrap_or(0.0))
        .fetch_one(&self.pool)
        .await
    }
}
