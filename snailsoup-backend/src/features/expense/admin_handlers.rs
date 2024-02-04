use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::expense::{ExpenseService, ExpenseServiceCreateError, ExpenseServiceGetError};

use super::api::{
    CategoryResponse, CreateCategoryRequest, CreateTagRequest, ExpenseResponse,
    FullExpenseResponse, TagResponse,
};

#[utoipa::path(
    get,
    path = "/api/admin/expenses/{expense_id}",
    tag = "Expenses - Admin",
    responses(
        (status = OK, description = "Expense found successfully", body = FullExpenseResponse),
        (status = NOT_FOUND, description = "Expense not found")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_expense_by_id(
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expense = service.get_expense(expense_id).await.map_err(|e| match e {
        ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match expense {
        Some(e) => Ok(Json(FullExpenseResponse::from(e))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/expenses",
    tag = "Expenses - Admin",
    responses(
        (status = OK, description = "Expense found successfully", body = [ExpenseResponse]),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_user_expenses(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expenses_opt = service
        .get_user_expenses(user_id)
        .await
        .map_err(|e| match e {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let expenses: Vec<ExpenseResponse> = match expenses_opt {
        None => Err(StatusCode::NOT_FOUND)?,
        Some(expenses) => expenses
            .into_iter()
            .map(|e| ExpenseResponse::from(e))
            .collect(),
    };

    Ok(Json(expenses))
}

#[utoipa::path(
    get,
    path = "/api/admin/expenses",
    tag = "Expenses - Admin",
    responses(
        (status = OK, description = "list expenses successfully", body = [ExpenseResponse])
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_all_expenses(
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let expenses: Vec<ExpenseResponse> = service
        .get_all_expenses()
        .await
        .map_err(|e| match e {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .into_iter()
        .map(|e| ExpenseResponse::from(e))
        .collect();

    Ok(Json(expenses))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/tags",
    tag = "Expenses - Admin",
    responses(
        (status = OK, description = "list tags successfully", body = [TagResponse]),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_tags_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let tags_opt = service
        .get_all_tags(user_id)
        .await
        .map_err(|err| match err {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
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
    tag = "Expenses - Admin",
    responses(
        (status = OK, description = "list tags successfully", body = [CategoryResponse]),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_categories_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<impl IntoResponse, StatusCode> {
    let categories_opt = service
        .get_all_categories(user_id)
        .await
        .map_err(|err| match err {
            ExpenseServiceGetError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
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

#[utoipa::path(
    post,
    path = "/api/admin/users/{user_id}/categories",
    tag = "Expenses - Admin",
    request_body = CreateCategoryRequest,
    responses(
        (status = CREATED, description = "tag created successfully", body = Uuid),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_create_category(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_category = service
        .create_category(user_id, body.name.as_str())
        .await
        .map_err(|e| match e {
            ExpenseServiceCreateError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ExpenseServiceCreateError::NoUser => StatusCode::NOT_FOUND,
            ExpenseServiceCreateError::ValidationError(_) => StatusCode::BAD_REQUEST,
        })?;

    Ok(Json(new_category))
}

#[utoipa::path(
    post,
    path = "/api/admin/users/{user_id}/tags",
    tag = "Expenses - Admin",
    request_body = CreateTagRequest,
    responses(
        (status = CREATED, description = "tag created successfully", body = Uuid),
        (status = NOT_FOUND, description = "user does not exists")
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_create_tag(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateTagRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let new_tag = service
        .create_tag(user_id, body.name.as_str())
        .await
        .map_err(|e| match e {
            ExpenseServiceCreateError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ExpenseServiceCreateError::NoUser => StatusCode::NOT_FOUND,
            ExpenseServiceCreateError::ValidationError(_) => StatusCode::BAD_REQUEST,
        })?;

    Ok(Json(new_tag))
}
