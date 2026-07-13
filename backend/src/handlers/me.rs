use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    app::AppState,
    auth::extractor::AuthenticatedUser,
    models::user::UpdateUserRequest,
    repository::user_repository::UserRepository,
};

pub async fn me(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
) -> impl IntoResponse {

    let repository = UserRepository::new(state.pool);

    match repository.find_by_id(user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update(
    AuthenticatedUser(user_id): AuthenticatedUser,
    State(state): State<AppState>,
    Json(request): Json<UpdateUserRequest>,
) -> impl IntoResponse {

    let repository = UserRepository::new(state.pool);

    match repository.update_user(user_id, request).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}