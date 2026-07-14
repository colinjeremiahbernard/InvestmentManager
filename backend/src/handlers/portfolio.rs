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
    models::portfolio::{
        AddPortfolioItemRequest, CreatePortfolioRequest, UpdatePortfolioItemRequest,
        UpdatePortfolioRequest,
    },
    repository::portfolio_repository::PortfolioRepository,
    services::portfolio_service::PortfolioService,
};

pub async fn create(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreatePortfolioRequest>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.create(user_id, request).await {
        Ok(p) => (StatusCode::CREATED, Json(p)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn list(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.list(user_id).await {
        Ok(p) => (StatusCode::OK, Json(p)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.get_detail(id, user_id).await {
        Ok(p) => (StatusCode::OK, Json(p)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

pub async fn update(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePortfolioRequest>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.update(id, user_id, request).await {
        Ok(p) => (StatusCode::OK, Json(p)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn delete(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.delete(id, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn add_item(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path(portfolio_id): Path<Uuid>,
    Json(request): Json<AddPortfolioItemRequest>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.add_item(portfolio_id, user_id, request).await {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn update_item(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path((portfolio_id, item_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdatePortfolioItemRequest>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service
        .update_item(portfolio_id, item_id, user_id, request)
        .await
    {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn remove_item(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Path((portfolio_id, item_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    let service = PortfolioService::new(PortfolioRepository::new(state.pool.clone()));
    match service.remove_item(portfolio_id, item_id, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
