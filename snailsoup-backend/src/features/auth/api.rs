use std::sync::Arc;

use axum::{Router, routing::post};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::services::auth::AuthService;

use super::handlers::login;

pub fn get_routes(service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .with_state(service)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema()]
    pub username: String,
    #[schema()]
    pub password: String,
}
