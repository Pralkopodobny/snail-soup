use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::expense::{Category, Expense, FullExpense, Tag};

pub struct ExpenseRepository {
    pool: Pool<Postgres>,
}

impl ExpenseRepository {
    pub fn new(pool: Pool<Postgres>) -> ExpenseRepository {
        ExpenseRepository { pool: pool }
    }

    pub async fn get_expense(&self, expense_id: Uuid) -> Result<Option<FullExpense>, sqlx::Error> {
        let expense = sqlx::query_as!(
            Expense,
            "
            SELECT *
            FROM expenses
            WHERE id = $1
            ",
            expense_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match expense {
            Some(e) => {
                let tags = sqlx::query_scalar!(
                    "
                    SELECT user_tag_id FROM expense_tags
                    WHERE expense_id = $1
                    ",
                    expense_id
                )
                .fetch_all(&self.pool)
                .await?;

                Ok(Some(FullExpense {
                    expense: e,
                    tags_ids: tags,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            Expense,
            "
            SELECT *
            FROM expenses
            "
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(expenses)
    }

    pub async fn get_all_tags_by_user_id(&self, user_id: Uuid) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query_as!(
            Tag,
            "
            SELECT *
            FROM user_tags 
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    pub async fn get_all_categories_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Category>, sqlx::Error> {
        let categories = sqlx::query_as!(
            Category,
            "
            SELECT *
            FROM user_categories 
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }
}
