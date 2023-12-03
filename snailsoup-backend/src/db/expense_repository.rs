use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::Expense;

pub struct ExpenseRepository {
    pool: Pool<Postgres>,
}

impl ExpenseRepository {
    pub fn new(pool: Pool<Postgres>) -> ExpenseRepository {
        ExpenseRepository { pool: pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Expense>, sqlx::Error> {
        let user = sqlx::query_as!(
            Expense,
            "
            SELECT * FROM expenses WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_all(&self) -> Result<Vec<Expense>, sqlx::Error> {
        let users = sqlx::query_as!(
            Expense,
            "
            SELECT * FROM expenses
            "
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }
}
