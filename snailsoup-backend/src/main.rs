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
async fn main() {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await
    {
        Ok(p) => p,
        Err(_) => panic!("Cannot connect to database!"),
    };

    println!("Connected to a database");

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));
    let expense_repository = Arc::new(db::ExpenseRepository::new(pool.clone()));

    let expense_service = Arc::new(ExpenseService::new(expense_repository.clone()));
    let user_service = Arc::new(UserService::new(app_user_repo.clone()));

    let config = Arc::new(Config::from("/api-doc.json"));

    warp::serve(openapi::openapi_filters(config).or(features::all_filters(
        expense_service.clone(),
        user_service.clone(),
    )))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
