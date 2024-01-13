mod app_state;
mod config;
mod db;
mod domain;
mod features;
mod services;

use std::sync::Arc;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::app_state::AppState;
use crate::config::Config;
use crate::services::auth::AuthService;
use crate::services::{ExpenseService, UserService};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await
    {
        Ok(p) => p,
        Err(_) => panic!("Cannot connect to database!"),
    };

    println!("Connected to a database");

    let config = Config::init();
    println!(
        "config read! JWT_MAXAGE:{}, JWT_EXPIRED_IN:{}",
        config.jwt_maxage, config.jwt_maxage
    );

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));
    let expense_repository = Arc::new(db::ExpenseRepository::new(pool.clone()));

    let expense_service = Arc::new(ExpenseService::new(expense_repository.clone()));
    let user_service = Arc::new(UserService::new(app_user_repo.clone()));
    let auth_service = Arc::new(AuthService::new(app_user_repo.clone()));

    let app_state = AppState::new(
        config,
        auth_service.clone(),
        user_service.clone(),
        expense_service.clone(),
    );

    let app = features::get_routes(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
