use std::sync::Arc;

use axum::Router;

use crate::services::{ExpenseService, UserService};

mod expense;
mod swagger;
mod user;

pub fn get_routes(expense_service: Arc<ExpenseService>, user_service: Arc<UserService>) -> Router {
    swagger::get_routes()
        .merge(expense::api::get_routes(expense_service).merge(user::api::get_routes(user_service)))
}
