mod config;
mod db;
mod domain;
mod features;
mod services;

use std::sync::Arc;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

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
    println!("config read! JWT_MAXAGE:{}, JWT_EXPIRED_IN:{}", config.jwt_maxage, config.jwt_maxage);

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));
    let expense_repository = Arc::new(db::ExpenseRepository::new(pool.clone()));

    let expense_service = Arc::new(ExpenseService::new(expense_repository.clone()));
    let user_service = Arc::new(UserService::new(app_user_repo.clone()));
    let auth_service = Arc::new(AuthService::new(app_user_repo.clone()));

    let app = features::get_routes(expense_service.clone(), user_service.clone(), auth_service.clone()).route_layer(
        axum::middleware::from_fn_with_state(user_service.clone(), xd),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn xd(
    axum::extract::State(user_service): axum::extract::State<Arc<UserService>>,
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, axum::Json<ErrorResponse>)>
{
    let user = user_service.get_all().await;
    if user.is_empty() {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Invalid token".to_string(),
        };
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::Json(json_error)))
    }

    req.extensions_mut().insert(user[0].clone());
    Ok(next.run(req).await)
    
}