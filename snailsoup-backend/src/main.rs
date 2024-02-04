mod app_state;
mod config;
mod db;
mod domain;
mod features;
mod services;

use std::sync::Arc;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::{
    app_state::AppState,
    config::Config,
    services::{auth::AuthService, expense::ExpenseService, user::UserService},
};

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
        config.jwt_maxage, config.jwt_expires_in
    );

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));
    let expense_repository = Arc::new(db::ExpenseRepository::new(pool.clone()));

    let app_state = AppState::new(
        config.clone(),
        Arc::new(AuthService::new(app_user_repo.clone(), config.clone())),
        Arc::new(UserService::new(app_user_repo.clone())),
        Arc::new(ExpenseService::new(
            expense_repository.clone(),
            app_user_repo.clone(),
        )),
    );

    let app = features::get_routes(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
