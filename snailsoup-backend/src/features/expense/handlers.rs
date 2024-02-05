use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    domain::app_user::AppUser,
    features::response::HttpError,
    services::expense::{ExpenseService, ExpenseServiceGetError},
    utils::{convert_to_vec, period::DatePeriod},
};

use super::api::{ExpenseResponse, FullExpenseResponse, TagResponse};

#[utoipa::path(
    get,
    path = "/api/expenses/{expense_id}",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, description = "Expense found successfully", body = FullExpenseResponse),
        (status = StatusCode::NOT_FOUND, description = "Expense not found"),
        (status = StatusCode::FORBIDDEN, description = "Not sufficient permissions"),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn expense_by_id(
    Extension(user): Extension<AppUser>,
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, HttpError> {
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

#[utoipa::path(
    get,
    path = "/api/expenses/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, description = "Expense found successfully", body = [ExpenseResponse]),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn expenses(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, HttpError> {
    let expenses: Vec<ExpenseResponse> = service
        .get_user_expenses(user.id)
        .await
        .map_err(|e| match e {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|e| ExpenseResponse::from(e))
        .collect();

    Ok(Json(expenses))
}

#[utoipa::path(
    get,
    path = "/api/expenses/query/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, description = "Expense found successfully", body = [ExpenseResponse]),
    ),
    params(
        (
            "from" = NaiveDate,
            Query,
            description = "Beginning of queried period (inclusive)",
        ),
        (
            "to" = NaiveDate,
            Query,
            description = "End of queried period (exclusive)",
        ),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn expenses_query(
    Extension(user): Extension<AppUser>,
    Query(period): Query<DatePeriod>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, HttpError> {
    if !period.is_valid() {
        Err(StatusCode::BAD_REQUEST)?
    }

    let expenses: Vec<ExpenseResponse> = service
        .get_user_expenses_in_period(user.id, period)
        .await
        .map_err(|e| match e {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|e| ExpenseResponse::from(e))
        .collect();

    Ok(Json(expenses))
}

#[utoipa::path(
    get,
    path = "/api/expense-tags/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, description = "Expense found successfully", body = [TagResponse]),
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn tags(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<TagResponse>>, HttpError> {
    service
        .get_all_tags(user.id)
        .await
        .map_err(|err| match err {
            ExpenseServiceGetError::InternalServerError => {
                HttpError::from(StatusCode::INTERNAL_SERVER_ERROR)
            }
        })?
        .ok_or(HttpError::from(StatusCode::INTERNAL_SERVER_ERROR))
        .map(|tags| {
            Json(convert_to_vec(tags, |t| TagResponse {
                id: t.id,
                name: t.name,
            }))
        })
}
