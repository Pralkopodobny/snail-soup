use crate::{domain, ExpenseService};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use utoipa::ToSchema;
use warp::hyper::StatusCode;

use warp::reply;

#[utoipa::path(
    get,
    path = "/api/admin/expenses/{id}",
    tag = "Expenses",
    responses(
        (status = 200, description = "Expense found succesfully", body = ExpenseResponse),
        (status = NOT_FOUND, description = "Expense not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Expense database id to get Expense for"),
    )
)]
pub async fn expense_by_id(
    id: uuid::Uuid,
    service: Arc<ExpenseService>,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let expense = service.get(id).await;

    match expense {
        Some(e) => Ok(Box::new(reply::json(&ExpenseResponse::from_expense(e)))),
        None => Ok(Box::new(reply::with_status(
            "Expense not found",
            StatusCode::NOT_FOUND,
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/expenses",
    tag = "Expenses",
    responses(
        (status = 200, description = "list expenses successfully", body = [ExpenseResponse])
    )
)]
pub async fn all_expenses(service: Arc<ExpenseService>) -> Result<impl warp::Reply, Infallible> {
    let expenses: Vec<ExpenseResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| ExpenseResponse::from_expense(e))
        .collect();

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
