use utoipa::OpenApi;

use super::SecurityAddon;

use crate::features::expense::handlers::__path_all_expenses;

#[derive(OpenApi)]
#[openapi(
            paths(all_expenses),
            components(
                schemas(crate::features::expense::handlers::ExpenseResponse)
            ),
            modifiers(&SecurityAddon),
            tags(
                (name = "Expenses", description = "Expense CRUD")
            )
        )]
pub struct ApiDoc;
