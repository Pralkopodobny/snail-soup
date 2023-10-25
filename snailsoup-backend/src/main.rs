use sqlx::postgres::PgPoolOptions;
mod db;
mod domain;

use crate::db::AppUserRepository;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup").await?;

    let kok = db::MyAppUserRepository{pool: pool};
    let user = kok.get(uuid::Uuid::parse_str("ca94889f-4375-4e28-b45c-8c23f12d86d4").unwrap()).await;

    let koko = match user {
        Ok(user_opt) => match user_opt {
            None => "no user with such id".to_string(),
            Some(user) => user.username
        },
        Err(_) => "Error!".to_string()
    };
    print!("\n{}\n", koko);

    Ok(())
}