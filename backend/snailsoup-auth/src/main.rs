mod app_state;
mod config;
mod db;
mod domain;
mod features;
mod services;

use std::sync::Arc;

use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Pool, Postgres};

use crate::{
    app_state::AppState,
    config::Config,
    services::{auth::AuthService, user::UserService},
};

async fn connect_to_db() -> Result<Pool<Postgres>, String> {
    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if !sqlx::Postgres::database_exists(connection_string.as_str())
        .await
        .map_err(|_| "Error checking if db exists")?
    {
        sqlx::Postgres::create_database(connection_string.as_str())
            .await
            .map_err(|_| "Error creating db")?;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str())
        .await
        .map_err(|_| "Cannot connect to database!")?;

    let migrations = if std::env::var("RUST_ENV") == Ok("production".to_string()) {
        // Productions migrations dir
        std::env::current_exe()
            .map_err(|e| e.to_string())?
            .parent()
            .ok_or("Unexpected Error when creating migrations path")?
            .join("./migrations")
    } else {
        // Development migrations dir
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?;
        std::path::Path::new(&crate_dir).join("./migrations")
    };

    println!("Trying to do a migration based on {}", migrations.display());

    sqlx::migrate::Migrator::new(migrations)
        .await
        .map_err(|e| e.to_string())?
        .run(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("Connected to a database");

    let pool = match connect_to_db().await {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    let config = Config::init();
    println!(
        "config read! JWT_MAXAGE:{}, JWT_EXPIRED_IN:{}",
        config.jwt_maxage, config.jwt_expires_in
    );

    let app_user_repo = Arc::new(db::AppUserRepository::new(pool.clone()));

    let app_state = AppState::new(
        config.clone(),
        Arc::new(AuthService::new(app_user_repo.clone(), config.clone())),
        Arc::new(UserService::new(app_user_repo.clone())),
    );

    let app = features::get_routes(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
