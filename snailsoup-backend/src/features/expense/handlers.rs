use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::expense::{ExpenseService, ExpenseServiceError};

use super::api::{CategoryResponse, ExpenseResponse, FullExpenseResponse, TagResponse};

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
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expense = service.get_expense(expense_id).await.map_err(|e| match e {
        ExpenseServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

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
pub(super) async fn all_expenses(
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expenses: Vec<ExpenseResponse> = service
        .get_all_expenses()
        .await
        .map_err(|e| match e {
            ExpenseServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .into_iter()
        .map(|e| ExpenseResponse::from_expense(e))
        .collect();

    Ok(Json(expenses))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/tags",
    tag = "Expenses",
    responses(
        (status = OK, description = "list tags successfully", body = [TagResponse]),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn tags_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let tags_opt = service
        .get_all_tags(user_id)
        .await
        .map_err(|err| match err {
            ExpenseServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let tags: Vec<TagResponse> = match tags_opt {
        None => Err(StatusCode::NOT_FOUND)?,
        Some(tags) => tags
            .into_iter()
            .map(|t| TagResponse {
                id: t.id,
                name: t.name,
            })
            .collect(),
    };

    Ok(Json(tags))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/categories",
    tag = "Expenses",
    responses(
        (status = OK, description = "list tags successfully", body = [CategoryResponse]),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn categories_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let categories_opt = service
        .get_all_categories(user_id)
        .await
        .map_err(|err| match err {
            ExpenseServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let categories: Vec<CategoryResponse> = match categories_opt {
        None => Err(StatusCode::NOT_FOUND)?,
        Some(tags) => tags
            .into_iter()
            .map(|t| CategoryResponse {
                id: t.id,
                name: t.name,
            })
            .collect(),
    };

    Ok(Json(categories))
}
