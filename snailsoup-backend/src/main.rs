use sqlx::postgres::PgPoolOptions;
mod db;
mod domain;
mod features;
mod openapi;
mod services;

use services::ExpenseService;

use std::sync::Arc;
use utoipa_swagger_ui::Config;
use warp::Filter;

use crate::services::UserService;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await?;

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));
    let expense_repository = Arc::new(db::ExpenseRepository::new(pool.clone()));
    let expense_service = Arc::new(ExpenseService::new(expense_repository.clone()));
    let user_service = Arc::new(UserService::new(app_user_repo.clone()));

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

    let config = Arc::new(Config::from("/api-doc.json"));

    warp::serve(
        openapi::openapi_filters(config)
            .or(hello)
            .or(features::all_filters(
                expense_service.clone(),
                user_service.clone(),
            )),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;

    Ok(())
}
