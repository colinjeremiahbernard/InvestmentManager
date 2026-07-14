use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error")]
    Internal,

    #[error("Asset already exists")]
    AssetAlreadyExists,

    #[error("Asset not found")]
    AssetNotFound,

    #[error("Portfolio not found")]
    PortfolioNotFound,

    #[error("Portfolio item not found")]
    PortfolioItemNotFound,

    #[error("Not authorized")]
    NotAuthorized,

     #[error("Transaction not found")]
    TransactionNotFound,
}
