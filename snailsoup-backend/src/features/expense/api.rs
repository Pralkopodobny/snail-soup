use std::sync::Arc;

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::handlers::{all_expenses, expense_by_id};
use crate::{domain, services::ExpenseService};

//TODO: secure them
pub fn get_admin_routes(service: Arc<ExpenseService>) -> Router {
    Router::new()
        .route("/api/admin/expenses", get(all_expenses))
        .route("/api/admin/expenses/:expense_id", get(expense_by_id))
        .with_state(service)
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
