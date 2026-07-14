use crate::models::user::{NewUser, UpdateUserRequest, User, UserResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
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
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
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
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }
    pub async fn find_by_id(&self, id: Uuid) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"
        SELECT
            id,
            first_name,
            last_name,
            username,
            email
        FROM users
        WHERE id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
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
            "#,
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .fetch_one(&self.pool)
        .await
    }
    pub async fn update_user(
        &self,
        id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"
        UPDATE users
        SET
            first_name=$1,
            last_name=$2,
            username=$3,
            email=$4
        WHERE id=$5
        RETURNING
            id,
            first_name,
            last_name,
            username,
            email
        "#,
            request.first_name,
            request.last_name,
            request.username,
            request.email,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
