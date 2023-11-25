use crate::{domain, services::UserService};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use utoipa::ToSchema;
use warp::hyper::StatusCode;

use warp::reply;

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
pub async fn user_by_id(
    id: uuid::Uuid,
    service: Arc<UserService>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let user = service.get(id).await;

    match user {
        Some(e) => Ok(Box::new(reply::json(&UserResponse::from_user(e)))),
        None => Ok(Box::new(reply::with_status(
            "User not found",
            StatusCode::NOT_FOUND,
        ))),
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
pub async fn all_users(service: Arc<UserService>) -> Result<impl warp::Reply, Infallible> {
    let users: Vec<UserResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| UserResponse::from_user(e))
        .collect();

    Ok(warp::reply::json(&users))
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
