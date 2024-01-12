use std::sync::Arc;

use crate::services::{auth::AuthService, ExpenseService, UserService};
use axum::http::{header, HeaderMap};
use axum::Router;

mod auth;
mod expense;
mod swagger;
mod user;

pub fn get_routes(
    expense_service: Arc<ExpenseService>,
    user_service: Arc<UserService>,
    auth_service: Arc<AuthService>,
) -> Router {
    let public_routes = expense::api::get_public_routes(expense_service.clone())
        .merge(user::api::get_public_routes(user_service.clone()))
        .merge(auth::api::get_public_routes(auth_service.clone()));

    let private_routes = user::api::get_private_routes(user_service.clone()).route_layer(
        axum::middleware::from_fn_with_state(auth_service.clone(), authorize),
    );

    swagger::get_routes()
        .merge(public_routes)
        .merge(private_routes)
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn authorize(
    headers: HeaderMap,
    //axum::extract::State(user_service): axum::extract::State<Arc<UserService>>,
    axum::extract::State(auth_service): axum::extract::State<Arc<AuthService>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, axum::Json<ErrorResponse>)>
{
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
            let json_error = ErrorResponse {
                message: "User does not exist".to_string(),
            };
            (axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error))
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

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
