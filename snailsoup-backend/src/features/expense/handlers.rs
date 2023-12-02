use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::api::ExpenseResponse;
use crate::services::ExpenseService;

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
pub(super) async fn expense_by_id(
    Path(expense_id): Path<uuid::Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expense = service.get(expense_id).await;

    match expense {
        Some(e) => Ok(Json(ExpenseResponse::from_expense(e))),
        None => Err(StatusCode::NOT_FOUND),
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
pub(super) async fn all_expenses(service: State<Arc<ExpenseService>>) -> impl IntoResponse {
    let expenses: Vec<ExpenseResponse> = service
        .get_all()
        .await
        .into_iter()
        .map(|e| ExpenseResponse::from_expense(e))
        .collect();

    Json(expenses)
}
