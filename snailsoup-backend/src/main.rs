use sqlx::postgres::PgPoolOptions;
mod db;
mod domain;
mod services;

use warp::Filter;


use services::expense::ExpenseService;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await?;

    let app_user_repo = db::AppUserRepository { pool: pool.clone() };
    let expense_repository = Arc::new(db::ExpenseRepository { pool: pool.clone() });
    let expense_service = Arc::new(ExpenseService {
        expense_repository: expense_repository.clone(),
    });

    let expense = expense_repository
        .get(uuid::Uuid::parse_str("5fe66f3f-a5a6-417e-957a-96508cd14736").unwrap())
        .await;

    let user = app_user_repo
        .get(uuid::Uuid::parse_str("ca94889f-4375-4e28-b45c-8c23f12d86d4").unwrap())
        .await;

    let date_message = match expense {
        Ok(expense_opt) => match expense_opt {
            None => "no expense with such id".to_string(),
            Some(expense) => expense.expense_date.to_string(),
        },
        Err(_) => "Error!".to_string(),
    };

    let user_message = match user {
        Ok(user_opt) => match user_opt {
            None => "no user with such id".to_string(),
            Some(user) => user.username,
        },
        Err(_) => "Error!".to_string(),
    };

    print!("\n{}\n", user_message);
    print!("\n{}\n", date_message);

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello.or(filters::all_filters(expense_service.clone())))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

mod filters {
    use std::sync::Arc;

    use super::handlers;
    use warp::Filter;
    use super::ExpenseService;

    /// The 4 TODOs filters combined.
    pub fn all_filters(
        db: Arc<ExpenseService>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        expense_by_id(db.clone()).or(all_expenses(db.clone()))
    }

    /// GET /todos?offset=3&limit=5
    pub fn expense_by_id(
        db: Arc<ExpenseService>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("expenses" / String)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::ask_for_expense)
    }

    pub fn all_expenses(
        db: Arc<ExpenseService>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("expenses")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::ask_for_all_expenses)
    }

    fn with_db(db: Arc<ExpenseService>) -> impl Filter<Extract = (Arc<ExpenseService>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use std::sync::Arc;
    use std::convert::Infallible;
    use super::ExpenseService;
    use warp::hyper::StatusCode;

    pub async fn ask_for_expense(id: String, db: Arc<ExpenseService>) -> Result<Box<dyn warp::Reply>, Infallible> {
        let parsed_id = match uuid::Uuid::parse_str(id.as_str()) {
            Ok(id) => id,
            Err(_) => return Ok(Box::new(StatusCode::BAD_REQUEST))
        };

        let expense = db
            .get(parsed_id)
            .await;

        Ok(Box::new(warp::reply::html(match expense {
            None => "no expense with such id".to_string(),
            Some(expense) => expense.cost.to_string(),
        })))
    }

    pub async fn ask_for_all_expenses(db: Arc<ExpenseService>) -> Result<impl warp::Reply, Infallible> {
        let expenses = db
            .get_all()
            .await;

        Ok(warp::reply::html(expenses.len().to_string()))
    }
}
