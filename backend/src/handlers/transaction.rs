use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;

use crate::{
    app::AppState,
    auth::extractor::AuthenticatedUser,
    models::transaction::{CreateTransactionRequest, UpdateTransactionRequest},
    repository::{
        portfolio_repository::PortfolioRepository, transaction_repository::TransactionRepository,
    },
    services::transaction_service::TransactionService,
};

pub async fn create(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    let service = TransactionService::new(
        TransactionRepository::new(state.pool.clone()),
        PortfolioRepository::new(state.pool.clone()),
    );

    match service.create(user_id, request).await {
        Ok(transaction) => (StatusCode::CREATED, Json(transaction)).into_response(),

        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn list(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let service = TransactionService::new(
        TransactionRepository::new(state.pool.clone()),
        PortfolioRepository::new(state.pool.clone()),
    );

    match service.list(user_id).await {
        Ok(transactions) => (StatusCode::OK, Json(transactions)).into_response(),

        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn get(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = TransactionService::new(
        TransactionRepository::new(state.pool.clone()),
        PortfolioRepository::new(state.pool.clone()),
    );

    match service.get(id, user_id).await {
        Ok(transaction) => (StatusCode::OK, Json(transaction)).into_response(),

        Err(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
    }
}

pub async fn update(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTransactionRequest>,
) -> impl IntoResponse {
    let service = TransactionService::new(
        TransactionRepository::new(state.pool.clone()),
        PortfolioRepository::new(state.pool.clone()),
    );

    match service.update(id, user_id, request).await {
        Ok(transaction) => (StatusCode::OK, Json(transaction)).into_response(),

        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn delete(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = TransactionService::new(
        TransactionRepository::new(state.pool.clone()),
        PortfolioRepository::new(state.pool.clone()),
    );

    match service.delete(id, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),

        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
