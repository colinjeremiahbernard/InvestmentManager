use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    app::AppState,
    models::user::CreateUserRequest,
    repository::user_repository::UserRepository,
    services::auth_service::AuthService,
};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let repository = UserRepository::new(state.pool.clone());

    let service = AuthService::new(repository);

    match service.register(request).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),

        Err(error) => (
            StatusCode::BAD_REQUEST,
            error.to_string(),
        )
            .into_response(),
    }
}