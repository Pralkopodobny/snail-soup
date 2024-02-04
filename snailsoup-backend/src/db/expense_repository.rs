use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    domain::expense::{Category, Expense, FullExpense, Tag},
    utils::period::DatePeriod,
};

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

    pub async fn get_all_expenses_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            Expense,
            "
            SELECT *
            FROM expenses
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(expenses)
    }

    pub async fn get_all_expenses_by_user_id_in_period(
        &self,
        user_id: Uuid,
        period: DatePeriod,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            Expense,
            "
            SELECT *
            FROM expenses
            WHERE user_id = $1 AND expense_date >= $2 AND expense_date < $3
            ",
            user_id,
            period.from,
            period.to
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

    pub async fn insert_tag(&self, tag: Tag) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_tags (id, user_id, name) VALUES ($1, $2, $3) RETURNING id
            "#,
            tag.id,
            tag.user_id,
            tag.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn delete_tag(&self, tag_id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            DELETE FROM user_tags WHERE id = $1 RETURNING id
            "#,
            tag_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(id)
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

    pub async fn insert_category(&self, category: Category) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_categories (id, user_id, name) VALUES ($1, $2, $3) RETURNING id
            "#,
            category.id,
            category.user_id,
            category.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn delete_category(&self, category_id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            DELETE FROM user_categories WHERE id = $1 RETURNING id
            "#,
            category_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn delete_category_force(
        &self,
        category_id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;
        sqlx::query!(
            "UPDATE expenses SET category_id = NULL WHERE category_id = $1",
            category_id
        )
        .execute(&mut *transaction)
        .await?;
        let id = sqlx::query_scalar!(
            r#"
            DELETE FROM user_categories WHERE id = $1 RETURNING id
            "#,
            category_id
        )
        .fetch_optional(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(id)
    }
}
