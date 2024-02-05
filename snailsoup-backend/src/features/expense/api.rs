use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    domain::expense::{Expense, FullExpense},
};

use super::admin_handlers;
use super::handlers;

pub fn get_admin_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/api/admin/expenses",
            get(admin_handlers::admin_all_expenses),
        )
        .route(
            "/api/admin/expenses/:expense_id",
            get(admin_handlers::admin_expense_by_id),
        )
        .route(
            "/api/admin/users/:user_id/tags",
            get(admin_handlers::admin_tags_by_user),
        )
        .route(
            "/api/admin/users/:user_id/categories",
            get(admin_handlers::admin_categories_by_user),
        )
        .route(
            "/api/admin/users/:user_id/categories",
            post(admin_handlers::admin_create_category),
        )
        .route(
            "/api/admin/users/:user_id/tags",
            post(admin_handlers::admin_create_tag),
        )
        .route(
            "/api/admin/users/:user_id/expenses",
            get(admin_handlers::admin_user_expenses),
        )
        .with_state(app_state)
}

pub fn get_private_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/expenses/:id", get(handlers::expense_by_id))
        .route("/api/expenses/", get(handlers::expenses))
        .route("/api/expenses/query/", get(handlers::expenses_query))
        .route("/api/expense-tags/", get(handlers::tags))
        .route("/api/expense-categories/", get(handlers::categories))
        .with_state(app_state)
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ExpenseResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub user: Uuid,
    #[schema()]
    pub category: Option<Uuid>,
    #[schema()]
    pub description: Option<String>,
    #[schema()]
    pub expense_date: chrono::NaiveDate,
    #[schema()]
    pub cost: rust_decimal::Decimal,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct FullExpenseResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub user: Uuid,
    #[schema()]
    pub category: Option<Uuid>,
    #[schema()]
    pub tags: Vec<Uuid>,
    #[schema()]
    pub description: Option<String>,
    #[schema()]
    pub expense_date: chrono::NaiveDate,
    #[schema()]
    pub cost: rust_decimal::Decimal,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CategoryResponse {
    #[schema()]
    pub id: Uuid,
    #[schema()]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateCategoryRequest {
    #[schema()]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateTagRequest {
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
            user: value.expense.user_id,
            category: value.expense.category_id,
            tags: value.tags_ids,
            description: value.expense.description,
            expense_date: value.expense.expense_date,
            cost: value.expense.cost,
        }
    }
}

impl From<Expense> for ExpenseResponse {
    fn from(value: Expense) -> Self {
        ExpenseResponse {
            id: value.id,
            user: value.user_id,
            category: value.category_id,
            description: value.description,
            expense_date: value.expense_date,
            cost: value.cost,
        }
    }
}
