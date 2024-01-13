use axum::{routing::post, Router};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::app_state::AppState;

use super::handlers::login;

pub fn get_public_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .with_state(app_state)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema()]
    pub username: String,
    #[schema()]
    pub password: String,
}
