use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::handlers::{all_expenses, expense_by_id};
use crate::{app_state::AppState, domain};

//TODO: secure them
pub fn get_admin_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/admin/expenses", get(all_expenses))
        .route("/api/admin/expenses/:expense_id", get(expense_by_id))
        .with_state(app_state)
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
    #[schema()]
    pub category: Option<CategoryResponse>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct FullExpenseResponse {
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
    #[schema()]
    pub category: Option<CategoryResponse>,
    #[schema()]
    pub tags: Vec<TagResponse>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CategoryResponse {
    #[schema()]
    pub id: uuid::Uuid,
    #[schema()]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TagResponse {
    #[schema()]
    pub id: uuid::Uuid,
    #[schema()]
    pub name: String,
}

impl FullExpenseResponse {
    pub fn from_expense(expense: domain::FullExpense) -> Self {
        FullExpenseResponse {
            id: expense.id,
            user_id: expense.user_id,
            description: expense.description,
            expense_date: expense.expense_date,
            cost: expense.cost,
            category: expense.category.map(|x| CategoryResponse {
                id: x.id,
                name: x.name,
            }),
            tags: expense
                .tags
                .into_iter()
                .map(|x| TagResponse {
                    id: x.id,
                    name: x.name,
                })
                .collect(),
        }
    }
}

impl ExpenseResponse {
    pub fn from_expense(expense: domain::Expense) -> Self {
        ExpenseResponse {
            id: expense.id,
            user_id: expense.user_id,
            description: expense.description,
            expense_date: expense.expense_date,
            cost: expense.cost,
            category: expense.category.map(|x| CategoryResponse {
                id: x.id,
                name: x.name,
            }),
        }
    }
}
