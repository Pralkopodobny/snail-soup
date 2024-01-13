use std::sync::Arc;

use crate::services::{auth::AuthService, ExpenseService, UserService};
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
    let public_routes = auth::api::get_public_routes(auth_service.clone());

    let private_routes = user::api::get_private_routes(user_service.clone()).route_layer(
        axum::middleware::from_fn_with_state(auth_service.clone(), auth::middleware::authorize),
    );

    let admin_routes = user::api::get_admin_routes(user_service.clone())
        .merge(expense::api::get_admin_routes(expense_service.clone()))
        .route_layer(axum::middleware::from_fn_with_state(
            auth_service.clone(),
            auth::middleware::authorize_admin,
        ));

    swagger::get_routes()
        .merge(public_routes)
        .merge(private_routes)
        .merge(admin_routes)
}
