use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::services::auth::{AuthService, LoginError, RegisterError};

use super::api::{LoginRequest, RegisterRequest};

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = OK, description = "list expenses successfully"),
        (status = UNAUTHORIZED, description = "user with such username and password does not exist"),
    )
)]
pub(super) async fn login(
    State(service): State<Arc<AuthService>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match service
        .login(body.username.as_str(), body.password.as_str())
        .await
    {
        Ok(token) => Ok(Json(token)),
        Err(e) => match e {
            LoginError::IncorrectUser => Err(StatusCode::UNAUTHORIZED),
            LoginError::IncorrectPassword => Err(StatusCode::UNAUTHORIZED),
            LoginError::InternalError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            LoginError::UnexpectedError => Err(StatusCode::INTERNAL_SERVER_ERROR),
            LoginError::InternalPasswordError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = CREATED, description = "user registered", body=Uuid),
        (status = BAD_REQUEST, description = "username already in use")
    )
)]
pub(super) async fn register(
    State(service): State<Arc<AuthService>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match service
        .register(body.username.as_str(), body.password.as_str())
        .await
    {
        Ok(u) => Ok((StatusCode::CREATED, Json(u.id)).into_response()),
        Err(e) => match e {
            RegisterError::UsernameInUse => Err(StatusCode::BAD_REQUEST),
            RegisterError::InternalError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
