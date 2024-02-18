use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::{
    features::response::HttpError,
    services::auth::{AuthService, LoginError, RegisterError},
};

use super::api::{LoginRequest, RegisterRequest};

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = OK),
        (status = UNAUTHORIZED, description = "User with provided username and password does not exist"),
    )
)]
pub(super) async fn login(
    State(service): State<Arc<AuthService>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, HttpError> {
    service
        .login(body.username.as_str(), body.password.as_str())
        .await
        .map_err(|e| match e {
            LoginError::IncorrectUser => HttpError::from(StatusCode::UNAUTHORIZED),
            LoginError::IncorrectPassword => HttpError::from(StatusCode::UNAUTHORIZED),
            LoginError::InternalError => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            LoginError::UnexpectedError => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            LoginError::InternalPasswordError => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })
        .map(|token| Json(token))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = CREATED, body=Uuid),
        (status = BAD_REQUEST)
    )
)]
pub(super) async fn register(
    State(service): State<Arc<AuthService>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, HttpError> {
    service
        .register(body.username.as_str(), body.password.as_str())
        .await
        .map_err(|e| match e {
            RegisterError::UsernameInUse => {
                HttpError::from((StatusCode::BAD_REQUEST, "Username is already used"))
            }
            RegisterError::InternalError => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })
        .map(|user| (StatusCode::CREATED, Json(user.id)))
}
