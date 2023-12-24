use std::sync::Arc;

use axum::{response::IntoResponse, http::StatusCode, Json, Router, extract::{Path, State}, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{services::UserService, domain};

pub fn get_routes(service: Arc<UserService>) -> Router {
    Router::new().route("/api/admin/users", get(all_users)).route("/api/admin/users/:user_id", get(user_by_id)).with_state(service)
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "User found successfully", body = UserResponse),
        (status = NOT_FOUND, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User database id to get User for"),
    )
)]
pub async fn user_by_id(
    Path(user_id): Path<uuid::Uuid>,
    service: State<Arc<UserService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = service.get(user_id).await;

    match user {
        Some(u) => Ok(Json(UserResponse::from_user(u))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/users",
    tag = "Users",
    responses(
        (status = 200, description = "list users successfully", body = [UserResponse])
    )
)]
pub async fn all_users(service: State<Arc<UserService>>) -> impl IntoResponse {
    let users: Vec<UserResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| UserResponse::from_user(e))
        .collect();

    Json(users)
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
