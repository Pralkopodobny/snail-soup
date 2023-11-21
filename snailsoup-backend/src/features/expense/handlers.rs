use crate::ExpenseService;
use std::convert::Infallible;
use std::sync::Arc;
use warp::hyper::StatusCode;

pub async fn expense_by_id(
    id: String,
    db: Arc<ExpenseService>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let parsed_id = match uuid::Uuid::parse_str(id.as_str()) {
        Ok(id) => id,
        Err(_) => return Ok(Box::new(StatusCode::BAD_REQUEST)),
    };

    let expense = db.get(parsed_id).await;

    Ok(Box::new(warp::reply::html(match expense {
        None => "no expense with such id".to_string(),
        Some(expense) => expense.cost.to_string(),
    })))
}

pub async fn all_expenses(db: Arc<ExpenseService>) -> Result<impl warp::Reply, Infallible> {
    let expenses = db.get_all().await;

    Ok(warp::reply::html(expenses.len().to_string()))
}
