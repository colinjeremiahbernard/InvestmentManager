use sqlx::PgPool;
use uuid::Uuid;

use crate::models::portfolio::{
    Portfolio, CreatePortfolioRequest,
    PortfolioItem, AddPortfolioItemRequest,
    UpdatePortfolioItemRequest, PortfolioItemResponse,
};

pub struct PortfolioRepository {
    pool: PgPool,
}

impl PortfolioRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreatePortfolioRequest,
    ) -> Result<Portfolio, sqlx::Error> {
        sqlx::query_as::<_, Portfolio>(
            r#"
            INSERT INTO portfolios (user_id, name, description)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, name, description, created_at, updated_at
            "#
        )
        .bind(user_id)
        .bind(request.name)
        .bind(request.description)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Portfolio>, sqlx::Error> {
        sqlx::query_as::<_, Portfolio>(
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM portfolios
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Portfolio>, sqlx::Error> {
        sqlx::query_as::<_, Portfolio>(
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM portfolios
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<Portfolio, sqlx::Error> {
        sqlx::query_as::<_, Portfolio>(
            r#"
            UPDATE portfolios
            SET name = $2, description = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, description, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM portfolios WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn add_item(
        &self,
        portfolio_id: Uuid,
        request: AddPortfolioItemRequest,
    ) -> Result<PortfolioItem, sqlx::Error> {
        sqlx::query_as::<_, PortfolioItem>(
            r#"
            INSERT INTO portfolio_items (portfolio_id, asset_id, quantity, purchase_price, notes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, portfolio_id, asset_id, quantity, purchase_price, notes, created_at, updated_at
            "#
        )
        .bind(portfolio_id)
        .bind(request.asset_id)
        .bind(request.quantity)
        .bind(request.purchase_price)
        .bind(request.notes)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_items(
        &self,
        portfolio_id: Uuid,
    ) -> Result<Vec<PortfolioItemResponse>, sqlx::Error> {
        sqlx::query_as::<_, PortfolioItemResponse>(
            r#"
            SELECT
                pi.id,
                pi.portfolio_id,
                pi.asset_id,
                a.symbol         AS asset_symbol,
                a.name           AS asset_name,
                a.asset_type,
                pi.quantity,
                pi.purchase_price,
                a.current_price,
                (pi.quantity * pi.purchase_price)                        AS total_invested,
                (pi.quantity * a.current_price)                          AS current_value,
                (pi.quantity * a.current_price)
                    - (pi.quantity * pi.purchase_price)                  AS gain_loss,
                CASE
                    WHEN (pi.quantity * pi.purchase_price) = 0 THEN 0.0
                    ELSE (
                        ((pi.quantity * a.current_price)
                            - (pi.quantity * pi.purchase_price))
                        / (pi.quantity * pi.purchase_price)
                    ) * 100.0
                END                                                      AS gain_loss_pct,
                pi.notes
            FROM portfolio_items pi
            JOIN assets a ON a.id = pi.asset_id
            WHERE pi.portfolio_id = $1
            ORDER BY pi.created_at DESC
            "#
        )
        .bind(portfolio_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_item_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PortfolioItem>, sqlx::Error> {
        sqlx::query_as::<_, PortfolioItem>(
            r#"
            SELECT id, portfolio_id, asset_id, quantity, purchase_price, notes, created_at, updated_at
            FROM portfolio_items
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_item(
        &self,
        id: Uuid,
        request: UpdatePortfolioItemRequest,
    ) -> Result<PortfolioItem, sqlx::Error> {
        sqlx::query_as::<_, PortfolioItem>(
            r#"
            UPDATE portfolio_items
            SET quantity = $2, purchase_price = $3, notes = $4, updated_at = NOW()
            WHERE id = $1
            RETURNING id, portfolio_id, asset_id, quantity, purchase_price, notes, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(request.quantity)
        .bind(request.purchase_price)
        .bind(request.notes)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_item(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM portfolio_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
