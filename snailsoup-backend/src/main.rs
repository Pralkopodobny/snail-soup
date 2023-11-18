use sqlx::postgres::PgPoolOptions;
mod db;
mod domain;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await?;

    let app_user_repo = db::AppUserRepository { pool: &pool };
    let expense_repository = db::ExpenseRepository { pool: &pool };

    let user = app_user_repo
        .get(uuid::Uuid::parse_str("ca94889f-4375-4e28-b45c-8c23f12d86d4").unwrap())
        .await;

    let expense = expense_repository
        .get(uuid::Uuid::parse_str("5fe66f3f-a5a6-417e-957a-96508cd14736").unwrap())
        .await;

    let user_message = match user {
        Ok(user_opt) => match user_opt {
            None => "no user with such id".to_string(),
            Some(user) => user.username,
        },
        Err(_) => "Error!".to_string(),
    };

    let date_message = match expense {
        Ok(expense_opt) => match expense_opt {
            None => "no expense with such id".to_string(),
            Some(expense) => expense.expense_date.to_string(),
        },
        Err(_) => "Error!".to_string(),
    };

    print!("\n{}\n", user_message);
    print!("\n{}\n", date_message);

    Ok(())
}
