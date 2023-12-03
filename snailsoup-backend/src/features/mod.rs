use std::sync::Arc;

use axum::Router;

use crate::services::{auth::AuthService, ExpenseService, UserService};

mod auth;
mod expense;
mod swagger;
mod user;

pub fn get_routes(
    expense_service: Arc<ExpenseService>,
    user_service: Arc<UserService>,
    auth_service: Arc<AuthService>,
) -> Router {
    swagger::get_routes()
        .merge(expense::api::get_routes(expense_service).merge(user::api::get_routes(user_service)))
        .merge(auth::api::get_routes(auth_service))
}
