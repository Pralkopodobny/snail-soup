use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    db::schema::{CategorySchema, ExpenseSchema, TagSchema},
    domain::expense::{Category, Expense, FullExpense, FullExpenseData, Tag},
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
        let expense: Option<Expense> = sqlx::query_as!(
            ExpenseSchema,
            "
            SELECT *
            FROM expenses
            WHERE id = $1
            ",
            expense_id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|e| e.into());

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
                    id: e.id,
                    data: FullExpenseData {
                        expense: e.data,
                        tags_ids: tags,
                    },
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses: Vec<Expense> = sqlx::query_as!(
            ExpenseSchema,
            "
            SELECT *
            FROM expenses
            "
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        Ok(expenses)
    }

    pub async fn get_all_expenses_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            ExpenseSchema,
            "
            SELECT *
            FROM expenses
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        Ok(expenses)
    }

    pub async fn get_all_expenses_by_user_id_in_period(
        &self,
        user_id: Uuid,
        period: DatePeriod,
    ) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            ExpenseSchema,
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
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        Ok(expenses)
    }

    pub async fn get_all_tags_by_user_id(&self, user_id: Uuid) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query_as!(
            TagSchema,
            "
            SELECT *
            FROM user_tags 
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        Ok(tags)
    }

    pub async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>, sqlx::Error> {
        let tag = sqlx::query_as!(
            TagSchema,
            "
            SELECT *
            FROM user_tags 
            WHERE id = $1
            ",
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|e| e.into());

        Ok(tag)
    }

    pub async fn insert_tag(&self, tag: Tag) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_tags (id, user_id, name) VALUES ($1, $2, $3) RETURNING id
            "#,
            tag.id,
            tag.data.user_id,
            tag.data.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn update_tag(&self, tag: Tag) -> Result<Option<Uuid>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            UPDATE user_tags SET name = $1, user_id = $2 WHERE id = $3 RETURNING id
            "#,
            tag.data.name,
            tag.data.user_id,
            tag.id
        )
        .fetch_optional(&self.pool)
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
            CategorySchema,
            "
            SELECT *
            FROM user_categories 
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        Ok(categories)
    }

    pub async fn get_category(&self, id: Uuid) -> Result<Option<Category>, sqlx::Error> {
        let category = sqlx::query_as!(
            CategorySchema,
            "
            SELECT *
            FROM user_categories 
            WHERE id = $1
            ",
            id,
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|e| e.into());

        Ok(category)
    }

    pub async fn insert_category(&self, category: Category) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO user_categories (id, user_id, name) VALUES ($1, $2, $3) RETURNING id
            "#,
            category.id,
            category.data.user_id,
            category.data.name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn update_category(&self, category: Category) -> Result<Option<Uuid>, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            UPDATE user_categories SET name = $1, user_id = $2 WHERE id = $3 RETURNING id
            "#,
            category.data.name,
            category.data.user_id,
            category.id
        )
        .fetch_optional(&self.pool)
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
