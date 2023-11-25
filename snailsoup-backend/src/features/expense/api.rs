use super::handlers;
use crate::ExpenseService;
use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;

pub fn expense_filters(
    service: Arc<ExpenseService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    expense_by_id(service.clone()).or(all_expenses(service.clone()))
}

fn expense_by_id(
    service: Arc<ExpenseService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "admin" / "expenses" / Uuid)
        .and(warp::get())
        .and(with_service(service))
        .and_then(handlers::expense_by_id)
}

fn all_expenses(
    service: Arc<ExpenseService>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("api" / "admin" / "expenses")
        .and(warp::get())
        .and(with_service(service))
        .and_then(handlers::all_expenses)
}

fn with_service(
    service: Arc<ExpenseService>,
) -> impl Filter<Extract = (Arc<ExpenseService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}
