use axum::http::{header, HeaderMap};
use std::sync::Arc;

use crate::{domain::app_user::AppUser, services::auth::AuthService};

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn authorize(
    headers: HeaderMap,
    axum::extract::State(auth_service): axum::extract::State<Arc<AuthService>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, axum::Json<ErrorResponse>)>
{
    let user = process_token(headers, auth_service).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn authorize_admin(
    headers: HeaderMap,
    axum::extract::State(auth_service): axum::extract::State<Arc<AuthService>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, axum::Json<ErrorResponse>)>
{
    let user = process_token(headers, auth_service).await?;

    if user.account_role != "Admin" {
        let json_error = ErrorResponse {
            message: "Insufficient privileges".to_string(),
        };
        return Err((axum::http::StatusCode::FORBIDDEN, axum::Json(json_error)));
    }

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

async fn process_token(
    headers: HeaderMap,
    auth_service: Arc<AuthService>,
) -> Result<AppUser, (axum::http::StatusCode, axum::Json<ErrorResponse>)> {
    if !headers.contains_key(header::AUTHORIZATION) {
        let json_error = ErrorResponse {
            message: "Missing authorization token".to_string(),
        };
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error)));
    }

    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let user = match token {
        None => {
            let json_error = ErrorResponse {
                message: "Invalid token".to_string(),
            };
            Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error)))
        }?,
        Some(val) => auth_service.auth_bearer_token(val.as_str()).await,
    }
    .map_err(|e| match e {
        crate::services::auth::AuthError::InvalidToken => {
            let json_error = ErrorResponse {
                message: "Invalid token".to_string(),
            };
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error))
        }
        crate::services::auth::AuthError::ExpiredToken => {
            let json_error = ErrorResponse {
                message: "Expired token".to_string(),
            };
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error))
        }
        crate::services::auth::AuthError::UserDoesNotExist => {
            let error = ErrorResponse {
                message: "User does not exist".to_string(),
            };
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(error))
        }
        crate::services::auth::AuthError::InternalError => {
            let json_error = ErrorResponse {
                message: "Internal error".to_string(),
            };
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json_error),
            )
        }
    })?;
    Ok(user)
}
