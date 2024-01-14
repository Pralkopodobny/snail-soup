use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::api::{ExpenseResponse, FullExpenseResponse};
use crate::services::ExpenseService;

#[utoipa::path(
    get,
    path = "/api/admin/expenses/{id}",
    tag = "Expenses",
    responses(
        (status = OK, description = "Expense found successfully", body = FullExpenseResponse),
        (status = NOT_FOUND, description = "Expense not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Expense database id to get Expense for"),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn expense_by_id(
    Path(expense_id): Path<uuid::Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expense = service.get(expense_id).await;

    match expense {
        Some(e) => Ok(Json(FullExpenseResponse::from_expense(e))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/expenses",
    tag = "Expenses",
    responses(
        (status = OK, description = "list expenses successfully", body = [ExpenseResponse])
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn all_expenses(service: State<Arc<ExpenseService>>) -> impl IntoResponse {
    let expenses: Vec<ExpenseResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| ExpenseResponse::from_expense(e))
        .collect();

    Json(expenses)
}
