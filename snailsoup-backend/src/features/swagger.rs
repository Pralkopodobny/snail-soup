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
use crate::features::user::handlers::__path_user_by_id;

pub fn get_routes() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
}

#[derive(OpenApi)]
#[openapi(
            paths(all_expenses, expense_by_id, all_users, user_by_id),
            components(
                schemas(super::expense::api::ExpenseResponse, super::user::api::UserResponse)
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
