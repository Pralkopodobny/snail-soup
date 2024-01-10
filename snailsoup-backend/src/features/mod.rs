use std::sync::Arc;

use crate::services::{auth::AuthService, ExpenseService, UserService};
use axum::http::HeaderMap;
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
        axum::middleware::from_fn_with_state(user_service.clone(), authorize),
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
    axum::extract::State(user_service): axum::extract::State<Arc<UserService>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, axum::Json<ErrorResponse>)>
{
    if !headers.contains_key("authorization") {
        let json_error = ErrorResponse {
            message: "Missing authorization token".to_string(),
        };
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error)));
    }

    let user = user_service.get_all().await;
    if user.is_empty() {
        let json_error = ErrorResponse {
            message: "Invalid token".to_string(),
        };
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error)));
    }

    req.extensions_mut().insert(user[0].clone());
    Ok(next.run(req).await)
}
