use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::Expense;

pub struct ExpenseRepository {
    pub pool: Pool<Postgres>,
}

impl ExpenseRepository {
    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<Expense>, Error> {
        let user = sqlx::query_as!(
            Expense,
            "
            SELECT id, user_id, description, expense_date, cost FROM expenses WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}