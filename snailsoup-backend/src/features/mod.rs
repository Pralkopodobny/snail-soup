pub mod expense;
pub mod user;

use crate::services::{ExpenseService, UserService};

use warp::Filter;

use std::sync::Arc;

pub fn all_filters(
    expense_service: Arc<ExpenseService>,
    user_service: Arc<UserService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    expense::expense_filters(expense_service).or(user::user_filters(user_service))
}
