use super::handlers;
use crate::services::UserService;
use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;

pub fn user_filters(
    service: Arc<UserService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    user_by_id(service.clone()).or(all_users(service.clone()))
}

fn user_by_id(
    service: Arc<UserService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "admin" / "users" / Uuid)
        .and(warp::get())
        .and(with_service(service))
        .and_then(handlers::user_by_id)
}

fn all_users(
    service: Arc<UserService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "admin" / "users")
        .and(warp::get())
        .and(with_service(service))
        .and_then(handlers::all_users)
}

fn with_service(
    service: Arc<UserService>,
) -> impl Filter<Extract = (Arc<UserService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}
