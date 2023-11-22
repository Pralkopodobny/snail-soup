pub mod expense;

use crate::ExpenseService;

use warp::Filter;

use std::sync::Arc;

pub fn all_filters(
    expense_service: Arc<ExpenseService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    expense::expense_filters(expense_service)
}

pub use expense::all_expenses;