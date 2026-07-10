use password_auth::{generate_hash, verify_password};
use crate::auth::jwt::create_token;
use crate::{
    error::app_error::AppError,
    models::user::{
        CreateUserRequest,
        LoginRequest,
        LoginResponse,
        NewUser,
        UserResponse,
    },
    repository::user_repository::UserRepository,
};

pub struct AuthService {
    repository: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(
        repository: UserRepository,
        jwt_secret: String,
    ) -> Self {
        Self { repository, jwt_secret, }
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
        pub async fn login(
        &self,
        request: LoginRequest,
    ) -> Result<LoginResponse, AppError> {

        let user = self
            .repository
            .find_by_email(&request.email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        verify_password(
            request.password,
            &user.password_hash,
        )
        .map_err(|_| AppError::InvalidCredentials)?;

       let token = create_token(
            user.id,
            &self.jwt_secret,
)
            .map_err(|_| AppError::Internal)?;

        Ok(LoginResponse {
            user: UserResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            email: user.email,
       },
        token,
})
    }
}