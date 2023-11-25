use utoipa::OpenApi;

use super::SecurityAddon;

use crate::features::expense::handlers::__path_all_expenses;
use crate::features::expense::handlers::__path_expense_by_id;

#[derive(OpenApi)]
#[openapi(
            paths(all_expenses, expense_by_id),
            components(
                schemas(crate::features::expense::handlers::ExpenseResponse)
            ),
            modifiers(&SecurityAddon),
            tags(
                (name = "Expenses", description = "Expense CRUD")
            )
        )]
pub struct ApiDoc;
