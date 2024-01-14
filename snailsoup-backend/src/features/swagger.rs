use axum::Router;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::features::expense::handlers::__path_all_expenses;
use crate::features::expense::handlers::__path_expense_by_id;

use crate::features::user::handlers::__path_all_users;
use crate::features::user::handlers::__path_me;
use crate::features::user::handlers::__path_user_by_id;

use crate::features::auth::handlers::__path_login;
use crate::features::auth::handlers::__path_register;

pub fn get_routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}

#[derive(OpenApi)]
#[openapi(
            paths(all_expenses, expense_by_id, all_users, user_by_id, login, register, me),
            components(
                schemas(
                    super::expense::api::ExpenseResponse,
                    super::expense::api::FullExpenseResponse,
                    super::expense::api::CategoryResponse,
                    super::expense::api::TagResponse,
                    super::user::api::UserResponse,
                    super::auth::api::LoginRequest,
                    super::auth::api::RegisterRequest)
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
