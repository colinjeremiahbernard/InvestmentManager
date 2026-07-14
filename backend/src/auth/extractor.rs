use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
};

use uuid::Uuid;

use crate::{app::AppState, auth::jwt::verify_token};

pub struct AuthenticatedUser(pub Uuid);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims =
            verify_token(token, &state.jwt_secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser(user_id))
    }
}
