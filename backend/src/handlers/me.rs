use axum::Json;

use crate::auth::extractor::AuthenticatedUser;

pub async fn me(
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user_id": user_id
    }))
}
