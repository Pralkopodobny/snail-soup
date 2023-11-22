use crate::{domain, ExpenseService};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use utoipa::ToSchema;
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

#[utoipa::path(
    get,
    path = "/api/admin/expenses",
    tag = "Expenses",
    responses(
        (status = 200, description = "list expenses successfully", body = [ExpenseResponse])
    )
)]
pub async fn all_expenses(db: Arc<ExpenseService>) -> Result<impl warp::Reply, Infallible> {
    let expenses : Vec<ExpenseResponse> = db.get_all().await.into_iter().map(|e| ExpenseResponse::from_expense(e)).collect();

    Ok(warp::reply::json(&expenses))
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ExpenseResponse {
    #[schema()]
    pub id: uuid::Uuid,
    #[schema()]
    pub user_id: uuid::Uuid,
    #[schema()]
    pub description: Option<String>,
    #[schema()]
    pub expense_date: chrono::NaiveDate,
    #[schema()]
    pub cost: rust_decimal::Decimal,
}

impl ExpenseResponse {
    pub fn from_expense(expense: domain::Expense) -> ExpenseResponse {
        ExpenseResponse {
            id: expense.id,
            user_id: expense.user_id,
            description: expense.description,
            expense_date: expense.expense_date,
            cost: expense.cost,
        }
    }
}
