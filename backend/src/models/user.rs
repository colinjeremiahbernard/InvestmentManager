use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Incoming registration request
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

// Login request
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Response sent to the frontend
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}