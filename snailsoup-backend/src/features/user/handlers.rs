use axum::{response::IntoResponse, Extension, Json};

use crate::domain::app_user::AppUser;

use super::api::UserResponse;

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
