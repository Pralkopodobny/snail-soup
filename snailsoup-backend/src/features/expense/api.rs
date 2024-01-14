use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    domain::expense::{Expense, FullExpense},
};

use super::handlers::{all_expenses, categories_by_user, expense_by_id, tags_by_user};

pub fn get_admin_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/admin/expenses", get(all_expenses))
        .route("/api/admin/expenses/:expense_id", get(expense_by_id))
        .route("/api/admin/users/:user_id/tags", get(tags_by_user))
        .route(
            "/api/admin/users/:user_id/categories",
            get(categories_by_user),
        )
        .with_state(app_state)
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ExpenseResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub user_id: Uuid,
    #[schema()]
    pub description: Option<String>,
    #[schema()]
    pub expense_date: chrono::NaiveDate,
    #[schema()]
    pub cost: rust_decimal::Decimal,
    #[schema()]
    pub category: Option<Uuid>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct FullExpenseResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub user_id: Uuid,
    #[schema()]
    pub description: Option<String>,
    #[schema()]
    pub expense_date: chrono::NaiveDate,
    #[schema()]
    pub cost: rust_decimal::Decimal,
    #[schema()]
    pub category: Option<Uuid>,
    #[schema()]
    pub tags: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CategoryResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TagResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub name: String,
}

impl From<FullExpense> for FullExpenseResponse {
    fn from(value: FullExpense) -> Self {
        FullExpenseResponse {
            id: value.expense.id,
            user_id: value.expense.user_id,
            description: value.expense.description,
            expense_date: value.expense.expense_date,
            cost: value.expense.cost,
            category: value.expense.category_id,
            tags: value.tags_ids,
        }
    }
}

impl From<Expense> for ExpenseResponse {
    fn from(value: Expense) -> Self {
        ExpenseResponse {
            id: value.id,
            user_id: value.user_id,
            description: value.description,
            expense_date: value.expense_date,
            cost: value.cost,
            category: value.category_id,
        }
    }
}
