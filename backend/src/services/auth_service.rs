use password_auth::generate_hash;

use crate::{
    error::app_error::AppError,
    models::user::{
        CreateUserRequest,
        NewUser,
        UserResponse,
    },
    repository::user_repository::UserRepository,
};

pub struct AuthService {
    repository: UserRepository,
}

impl AuthService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn register(
        &self,
        request: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {

        if self
            .repository
            .find_by_email(&request.email)
            .await?
            .is_some()
        {
            return Err(AppError::EmailAlreadyExists);
        }

        if self
            .repository
            .find_by_username(&request.username)
            .await?
            .is_some()
        {
            return Err(AppError::UsernameAlreadyExists);
        }

        let password_hash = generate_hash(request.password);

        let user = NewUser {
            first_name: request.first_name,
            last_name: request.last_name,
            username: request.username,
            email: request.email,
            password_hash,
        };

        let created = self.repository.create_user(user).await?;

        Ok(UserResponse {
            id: created.id,
            first_name: created.first_name,
            last_name: created.last_name,
            username: created.username,
            email: created.email,
        })
    }
}