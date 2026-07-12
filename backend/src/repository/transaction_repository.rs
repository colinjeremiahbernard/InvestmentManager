use sqlx::PgPool;
use uuid::Uuid;

use crate::models::transaction::{
    Transaction,
    CreateTransactionRequest,
    UpdateTransactionRequest,
};

pub struct TransactionRepository {
    pool: PgPool,
}

impl TransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (
                portfolio_id,
                asset_id,
                transaction_type,
                quantity,
                price,
                amount,
                notes
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            RETURNING
                id,
                portfolio_id,
                asset_id,
                transaction_type,
                quantity,
                price,
                amount,
                notes,
                created_at,
                updated_at
            "#
        )
        .bind(request.portfolio_id)
        .bind(request.asset_id)
        .bind(request.transaction_type)
        .bind(request.quantity)
        .bind(request.price)
        .bind(request.amount)
        .bind(request.notes)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Transaction>, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT
                id,
                portfolio_id,
                asset_id,
                transaction_type,
                quantity,
                price,
                amount,
                notes,
                created_at,
                updated_at
            FROM transactions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn list(
        &self,
        portfolio_id: Uuid,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT
                id,
                portfolio_id,
                asset_id,
                transaction_type,
                quantity,
                price,
                amount,
                notes,
                created_at,
                updated_at
            FROM transactions
            WHERE portfolio_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(portfolio_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction, sqlx::Error> {
        sqlx::query_as::<_, Transaction>(
            r#"
            UPDATE transactions
            SET
                quantity = $2,
                price = $3,
                amount = $4,
                notes = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                portfolio_id,
                asset_id,
                transaction_type,
                quantity,
                price,
                amount,
                notes,
                created_at,
                updated_at
            "#
        )
        .bind(id)
        .bind(request.quantity)
        .bind(request.price)
        .bind(request.amount)
        .bind(request.notes)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(
        &self,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM transactions WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}