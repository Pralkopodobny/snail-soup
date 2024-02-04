use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    domain::app_user::AppUser,
    services::expense::{ExpenseService, ExpenseServiceGetError},
};

use super::api::FullExpenseResponse;

#[utoipa::path(
    get,
    path = "/api/expenses/{expense_id}",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, description = "Expense found successfully", body = FullExpenseResponse),
        (status = StatusCode::NOT_FOUND, description = "Expense not found"),
        (status = StatusCode::FORBIDDEN, description = "Expense not found"),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn expense_by_id(
    Extension(user): Extension<AppUser>,
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expense = service.get_expense(expense_id).await.map_err(|e| match e {
        ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match expense {
        Some(e) => {
            if e.expense.user_id != user.id {
                Err(StatusCode::FORBIDDEN)?
            }
            Ok(Json(FullExpenseResponse::from(e)))
        }
        None => Err(StatusCode::NOT_FOUND)?,
    }
}
