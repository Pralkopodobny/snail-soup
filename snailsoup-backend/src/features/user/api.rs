use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{app_state::AppState, domain};

use super::handlers::{all_users, me, user_by_id};

pub fn get_admin_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/admin/users", get(all_users))
        .route("/api/admin/users/:user_id", get(user_by_id))
        .with_state(app_state)
}

pub fn get_private_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/users/me", get(me))
        .with_state(app_state)
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserResponse {
    #[schema()]
    pub id: uuid::Uuid,
    #[schema()]
    pub username: String,
    #[schema()]
    pub account_role: String,
}

impl UserResponse {
    pub fn from_user(user: domain::AppUser) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
            account_role: user.account_role,
        }
    }
}
