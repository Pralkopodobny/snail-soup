use axum::Router;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::features::expense::handlers::{
    __path_all_expenses, __path_categories_by_user, __path_create_category, __path_create_tag,
    __path_expense_by_id, __path_tags_by_user, __path_user_expenses,
};

use crate::features::user::handlers::{__path_all_users, __path_me, __path_user_by_id};

use crate::features::auth::handlers::{__path_login, __path_register};

pub fn get_routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}

#[derive(OpenApi)]
#[openapi(
            paths(
                login, register,
                all_expenses, expense_by_id, tags_by_user, categories_by_user, create_category, create_tag, user_expenses,
                all_users, user_by_id, me
            ),
            components(
                schemas(
                    super::expense::api::ExpenseResponse,
                    super::expense::api::FullExpenseResponse,
                    super::expense::api::CategoryResponse,
                    super::expense::api::TagResponse,
                    super::expense::api::CreateTagRequest,
                    super::expense::api::CreateCategoryRequest,
                    super::user::api::UserResponse,
                    super::auth::api::LoginRequest,
                    super::auth::api::RegisterRequest
                )
            ),
            modifiers(&SecurityAddon),
            tags(
                (name = "Expenses", description = "Expense CRUD")
            )
        )]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Bearer token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
