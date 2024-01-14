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
    pub category: Option<CategoryResponse>,
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
    pub category: Option<CategoryResponse>,
    #[schema()]
    pub tags: Vec<TagResponse>,
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

impl FullExpenseResponse {
    pub fn from_expense(expense: FullExpense) -> Self {
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
    pub fn from_expense(expense: Expense) -> Self {
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
