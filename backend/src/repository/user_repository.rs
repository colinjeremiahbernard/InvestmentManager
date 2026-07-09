use sqlx::PgPool;

use crate::models::user::{NewUser, User};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                first_name,
                last_name,
                username,
                email,
                password_hash,
                created_at,
                updated_at
            FROM users
            WHERE email = $1
            "#
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT
                id,
                first_name,
                last_name,
                username,
                email,
                password_hash,
                created_at,
                updated_at
            FROM users
            WHERE username = $1
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_user(
        &self,
        user: NewUser,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                first_name,
                last_name,
                username,
                email,
                password_hash
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                first_name,
                last_name,
                username,
                email,
                password_hash,
                created_at,
                updated_at
            "#
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .fetch_one(&self.pool)
        .await
    }
}