use utoipa::OpenApi;

use super::SecurityAddon;

use crate::features::expense::handlers::__path_all_expenses;
use crate::features::expense::handlers::__path_expense_by_id;

use crate::features::user::handlers::__path_all_users;
use crate::features::user::handlers::__path_user_by_id;

#[derive(OpenApi)]
#[openapi(
            paths(all_expenses, expense_by_id, all_users, user_by_id),
            components(
                schemas(crate::features::expense::handlers::ExpenseResponse, crate::features::user::handlers::UserResponse)
            ),
            modifiers(&SecurityAddon),
            tags(
                (name = "Expenses", description = "Expense CRUD")
            )
        )]
pub struct ApiDoc;
