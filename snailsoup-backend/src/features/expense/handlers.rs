use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        app_user::AppUser,
        expense::{Category, CategoryData, ExpenseData, FullExpenseData, Tag, TagData},
    },
    features::response::HttpError,
    services::expense::{CreateError, ExpenseService, GetError},
    utils::{convert_to_vec, period::DatePeriod},
};

use super::api::{
    CategoryResponse, CreateCategoryRequest, CreateExpenseRequest, CreateTagRequest,
    ExpenseResponse, FullExpenseResponse, TagResponse,
};

#[utoipa::path(
    get,
    path = "/api/expenses/{expense_id}",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, body = FullExpenseResponse),
        (status = StatusCode::NOT_FOUND),
        (status = StatusCode::FORBIDDEN),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn expense_by_id(
    Extension(user): Extension<AppUser>,
    Path(expense_id): Path<Uuid>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<FullExpenseResponse>, HttpError> {
    let expense = service.get_expense(expense_id).await.map_err(|e| match e {
        GetError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    match expense {
        Some(e) => {
            if e.data.expense.user_id != user.id {
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
        (status = StatusCode::OK, body = [ExpenseResponse]),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn expenses(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<ExpenseResponse>>, HttpError> {
    service
        .get_expenses_for_user(user.id)
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::INTERNAL_SERVER_ERROR))
        .map(|expenses| Json(convert_to_vec(expenses, |e| ExpenseResponse::from(e))))
}

#[utoipa::path(
    get,
    path = "/api/expenses/query/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, body = [ExpenseResponse]),
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
    security(("BearerToken" = []))
)]
pub(super) async fn expenses_query(
    Extension(user): Extension<AppUser>,
    Query(period): Query<DatePeriod>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<ExpenseResponse>>, HttpError> {
    if !period.is_valid() {
        Err(StatusCode::BAD_REQUEST)?
    }

    service
        .get_expenses_for_user_in_period(user.id, period)
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::INTERNAL_SERVER_ERROR))
        .map(|expenses| Json(convert_to_vec(expenses, |e| ExpenseResponse::from(e))))
}

#[utoipa::path(
    post,
    path = "/api/expenses/",
    tag = "Expenses",
    request_body = CreateExpenseRequest,
    responses(
        (status = StatusCode::OK, body = Uuid),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn add_expense(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateExpenseRequest>,
) -> Result<Json<Uuid>, HttpError> {
    service
        .create_expense(FullExpenseData {
            expense: ExpenseData {
                user_id: user.id,
                category_id: body.category,
                description: body.description,
                expense_date: body.expense_date,
                cost: body.cost,
            },
            tags_ids: body.tags.unwrap_or(vec![]),
        })
        .await
        .map_err(|e| match e {
            CreateError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            CreateError::NoUser => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
            CreateError::Validation(message) => {
                HttpError::from((StatusCode::BAD_REQUEST, message.as_str()))
            }
        })
        .map(|id| Json(id))
}

#[utoipa::path(
    get,
    path = "/api/expense-tags/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, body = [TagResponse]),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn tags(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<TagResponse>>, HttpError> {
    service
        .get_tags_for_user(user.id)
        .await
        .map_err(|err| match err {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::INTERNAL_SERVER_ERROR))
        .map(|tags| {
            Json(convert_to_vec(tags, |t| TagResponse {
                id: t.id,
                name: t.data.name,
            }))
        })
}

#[utoipa::path(
    post,
    path = "/api/expense-tags/",
    tag = "Expenses",
    request_body = CreateTagRequest,
    responses(
        (status = StatusCode::OK, body = Uuid),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn add_tag(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateTagRequest>,
) -> Result<Json<Uuid>, HttpError> {
    service
        .create_tag(TagData {
            user_id: user.id,
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
    patch,
    path = "/api/expense-tags/{tag_id}",
    tag = "Expenses",
    request_body = CreateTagRequest,
    responses(
        (status = StatusCode::OK, body = Uuid),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn update_tag(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
    Path(tag_id): Path<Uuid>,
    Json(body): Json<CreateTagRequest>,
) -> Result<Json<Uuid>, HttpError> {
    let tag = service
        .get_tag(tag_id)
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))?;

    if tag.data.user_id != user.id {
        Err(HttpError::from(StatusCode::FORBIDDEN))?
    }

    service
        .update_tag(Tag {
            id: tag_id,
            data: TagData {
                user_id: user.id,
                name: body.name,
            },
        })
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|id| Json(id))
}

#[utoipa::path(
    get,
    path = "/api/expense-categories/",
    tag = "Expenses",
    responses(
        (status = StatusCode::OK, body = [CategoryResponse]),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn categories(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
) -> Result<Json<Vec<CategoryResponse>>, HttpError> {
    service
        .get_categories_for_user(user.id)
        .await
        .map_err(|err| match err {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::INTERNAL_SERVER_ERROR))
        .map(|tags| {
            Json(convert_to_vec(tags, |t| CategoryResponse {
                id: t.id,
                name: t.data.name,
            }))
        })
}

#[utoipa::path(
    post,
    path = "/api/expense-categories/",
    tag = "Expenses",
    request_body = CreateCategoryRequest,
    responses(
        (status = StatusCode::OK, body = Uuid),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn add_category(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<Json<Uuid>, HttpError> {
    service
        .create_category(CategoryData {
            user_id: user.id,
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
    patch,
    path = "/api/expense-categories/{category_id}",
    tag = "Expenses",
    request_body = CreateCategoryRequest,
    responses(
        (status = StatusCode::OK, body = Uuid),
    ),
    security(("BearerToken" = []))
)]
pub(super) async fn update_category(
    Extension(user): Extension<AppUser>,
    service: State<Arc<ExpenseService>>,
    Path(category_id): Path<Uuid>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<Json<Uuid>, HttpError> {
    let category = service
        .get_category(category_id)
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))?;

    if category.data.user_id != user.id {
        Err(HttpError::from(StatusCode::FORBIDDEN))?
    }

    service
        .update_category(Category {
            id: category_id,
            data: CategoryData {
                user_id: user.id,
                name: body.name,
            },
        })
        .await
        .map_err(|e| match e {
            GetError::Internal => HttpError::from(StatusCode::INTERNAL_SERVER_ERROR),
        })?
        .ok_or(HttpError::from(StatusCode::NOT_FOUND))
        .map(|id| Json(id))
}
