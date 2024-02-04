use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{domain::app_user::AppUser, services::user::UserService};

use super::api::UserResponse;

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}",
    tag = "Users - Admin",
    responses(
        (status = OK, description = "User found successfully", body = UserResponse),
        (status = NOT_FOUND, description = "User not found")
    ),
    params(
        ("user_id" = Uuid, Path, description = "User database id to get User for"),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn user_by_id(
    Path(user_id): Path<Uuid>,
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
    tag = "Users - Admin",
    responses(
        (status = OK, description = "list users successfully", body = [UserResponse])
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn all_users(service: State<Arc<UserService>>) -> impl IntoResponse {
    let users: Vec<UserResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| UserResponse::from_user(e))
        .collect();

    Json(users)
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Users",
    responses(
        (status = OK, description = "User found successfully", body = UserResponse),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn me(Extension(user): Extension<AppUser>) -> impl IntoResponse {
    Json(UserResponse::from_user(user))
}
