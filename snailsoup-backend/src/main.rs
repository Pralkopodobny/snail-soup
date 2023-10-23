use sqlx::postgres::PgPoolOptions;

struct Expense {id: uuid::Uuid, expense_date: sqlx::types::time::Date, tag: String}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup").await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);
    print!("{}", row.0);

    

    let expenses = sqlx::query_as!(Expense,
        "
        SELECT id, expense_date, tag FROM expenses
        "
    ).fetch_all(&pool)
    .await?;

    for expense in expenses {
        print!("{}", expense.expense_date)
    }

    Ok(())
}