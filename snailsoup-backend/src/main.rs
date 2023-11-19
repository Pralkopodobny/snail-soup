use sqlx::postgres::PgPoolOptions;
mod db;
mod domain;

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/snailsoup")
        .await?;

    let app_user_repo = db::AppUserRepository { pool: pool.clone() };
    let expense_repository = db::ExpenseRepository { pool: pool.clone() };

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

    warp::serve(hello.or(filters::todos(app_user_repo.clone()))).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

mod filters {
    use std::sync::Arc;

    use super::handlers;
    use warp::Filter;
    use super::db;
    pub type Db = db::AppUserRepository;

    /// The 4 TODOs filters combined.
    pub fn todos(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        todos_list(db.clone())
    }

    /// GET /todos?offset=3&limit=5
    pub fn todos_list(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("todos" / String)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::ask_for_user)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use crate::db;
    use std::convert::Infallible;
    use warp::Filter;
    use super::filters::Db;

    pub async fn ask_for_user(
        id: String,
        db: Db,
    ) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let user = db
            .get(uuid::Uuid::parse_str("ca94889f-4375-4e28-b45c-8c23f12d86d4").unwrap())
            .await;

        Ok(warp::reply::html(match user {
            Ok(user_opt) => match user_opt {
                None => "no user with such id".to_string(),
                Some(user) => user.username,
            },
            Err(_) => "Error!".to_string(),
        }))
    }
}
