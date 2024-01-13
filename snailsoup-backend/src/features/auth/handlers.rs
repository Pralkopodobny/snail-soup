use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::services::auth::{AuthService, LoginError};

use super::api::LoginRequest;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "list expenses successfully"),
        (status = 401, description = "user with such username and password does not exist"),
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
            LoginError::InternalPasswordError => {
                println!("xd");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}
