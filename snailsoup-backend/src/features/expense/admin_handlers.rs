use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::expense::{CategoryData, TagData},
    features::response::HttpError,
    services::expense::{CreateError, ExpenseService, GetError},
    utils::convert_to_vec,
};

use super::api::{
    CategoryResponse, CreateCategoryRequest, CreateTagRequest, ExpenseResponse,
    FullExpenseResponse, TagResponse,
};

#[utoipa::path(
    get,
    path = "/api/admin/expenses/{expense_id}",
    tag = "Expenses - Admin",
    responses(
        (status = StatusCode::OK, body = FullExpenseResponse),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_expense_by_id(
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<FullExpenseResponse>, HttpError> {
    service
        .get_expense(expense_id)
        .await
        .map_err(|e| match e {
            GetError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|expense| Json(FullExpenseResponse::from(expense)))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/expenses",
    tag = "Expenses - Admin",
    responses(
        (status = StatusCode::OK, body = [ExpenseResponse]),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_user_expenses(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<ExpenseResponse>>, HttpError> {
    service
        .get_expenses_for_user(user_id)
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|expenses| Json(convert_to_vec(expenses, |e| ExpenseResponse::from(e))))
}

#[utoipa::path(
    get,
    path = "/api/admin/expenses",
    tag = "Expenses - Admin",
    responses(
        (status = StatusCode::OK, body = [ExpenseResponse])
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_all_expenses(
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<ExpenseResponse>>, HttpError> {
    service
        .get_all_expenses()
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })
        .map(|expenses| Json(convert_to_vec(expenses, |e| ExpenseResponse::from(e))))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/tags",
    tag = "Expenses - Admin",
    responses(
        (status = StatusCode::OK, body = [TagResponse]),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_tags_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<TagResponse>>, HttpError> {
    service
        .get_tags_for_user(user_id)
        .await
        .map_err(|err| match err {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|tags| {
            Json(convert_to_vec(tags, |t| TagResponse {
                id: t.id,
                name: t.data.name,
            }))
        })
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{user_id}/categories",
    tag = "Expenses - Admin",
    responses(
        (status = StatusCode::OK, body = [CategoryResponse]),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_categories_by_user(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<CategoryResponse>>, HttpError> {
    service
        .get_categories_for_user(user_id)
        .await
        .map_err(|err| match err {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|categories| {
            Json(convert_to_vec(categories, |c| CategoryResponse {
                id: c.id,
                name: c.data.name,
            }))
        })
}

#[utoipa::path(
    post,
    path = "/api/admin/users/{user_id}/categories",
    tag = "Expenses - Admin",
    request_body = CreateCategoryRequest,
    responses(
        (status = StatusCode::CREATED, body = Uuid),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_create_category(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<Json<Uuid>, HttpError> {
    service
        .create_category(CategoryData {
            user_id: user_id,
            name: body.name,
        })
        .await
        .map_err(|e| match e {
            CreateError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            CreateError::NoUser => HttpError::from(StatusCode::NOT_FOUND),
            CreateError::Validation(m) => HttpError::from((StatusCode::BAD_REQUEST, m.as_str())),
        })
        .map(|id| Json(id))
}

#[utoipa::path(
    post,
    path = "/api/admin/users/{user_id}/tags",
    tag = "Expenses - Admin",
    request_body = CreateTagRequest,
    responses(
        (status = StatusCode::CREATED, body = Uuid),
        (status = StatusCode::NOT_FOUND)
    ),
    security(("Bearer token" = []))
)]
pub(super) async fn admin_create_tag(
    Path(user_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateTagRequest>,
) -> Result<Json<Uuid>, HttpError> {
    service
        .create_tag(TagData {
            user_id: user_id,
            name: body.name,
        })
        .await
        .map_err(|e| match e {
            CreateError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            CreateError::NoUser => HttpError::from(StatusCode::NOT_FOUND),
            CreateError::Validation(m) => HttpError::from((StatusCode::BAD_REQUEST, m.as_str())),
        })
        .map(|id| Json(id))
}
