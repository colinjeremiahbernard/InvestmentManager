use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    app::AppState, auth::extractor::AuthenticatedUser, models::asset::CreateAssetRequest,
    repository::asset_repository::AssetRepository, services::asset_service::AssetService,
};

pub async fn create(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<CreateAssetRequest>,
) -> impl IntoResponse {
    let service = AssetService::new(AssetRepository::new(state.pool.clone()));

    match service.create(request).await {
        Ok(asset) => (StatusCode::CREATED, Json(asset)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn list(_user: AuthenticatedUser, State(state): State<AppState>) -> impl IntoResponse {
    let service = AssetService::new(AssetRepository::new(state.pool.clone()));

    match service.list().await {
        Ok(assets) => (StatusCode::OK, Json(assets)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn get(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = AssetService::new(AssetRepository::new(state.pool.clone()));

    match service.get(id).await {
        Ok(Some(asset)) => (StatusCode::OK, Json(asset)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Asset not found").into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn update(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateAssetRequest>,
) -> impl IntoResponse {
    let service = AssetService::new(AssetRepository::new(state.pool.clone()));

    match service.update(id, request).await {
        Ok(asset) => (StatusCode::OK, Json(asset)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

pub async fn delete(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let service = AssetService::new(AssetRepository::new(state.pool.clone()));

    match service.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
