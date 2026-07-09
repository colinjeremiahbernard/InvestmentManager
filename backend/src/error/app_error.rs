use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Password hashing failed")]
    PasswordHashing,

    #[error("Internal server error")]
    Internal,
}