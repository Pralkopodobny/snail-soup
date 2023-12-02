use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::services::UserService;

use super::api::UserResponse;

#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "User found succesfully", body = UserResponse),
        (status = NOT_FOUND, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User database id to get User for"),
    )
)]
pub(super) async fn user_by_id(
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
pub(super) async fn all_users(service: State<Arc<UserService>>) -> impl IntoResponse {
    let users: Vec<UserResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| UserResponse::from_user(e))
        .collect();

    Json(users)
}
