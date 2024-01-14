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
            schema::ExpenseWithCategoryDb,
            r#"
            SELECT e.id, e.user_id, e.category_id, c.name "category_name", e.description, e.expense_date, e.cost
            FROM expenses e 
            LEFT JOIN user_categories c ON e.category_id = c.id 
            WHERE e.id = $1
            "#,
            expense_id
        )
        .fetch_optional(&self.pool)
        .await?;

        match expense {
            Some(e) => {
                let tags = sqlx::query_as!(
                    schema::TagDb,
                    "
                    SELECT ut.id, ut.name FROM expense_tags et 
                    JOIN user_tags ut ON et.user_tag_id = ut.id 
                    WHERE et.expense_id = $1
                    ",
                    expense_id
                )
                .fetch_all(&self.pool)
                .await?;
                Ok(Some(FullExpense::from_db(e, tags)))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query_as!(
            schema::ExpenseWithCategoryDb,
            r#"
            SELECT e.id, e.user_id, e.category_id, c.name "category_name", e.description, e.expense_date, e.cost
            FROM expenses e 
            LEFT JOIN user_categories c ON e.category_id = c.id 
            "#
        )
        .fetch_all(&self.pool)
        .await?.into_iter().map(|e| Expense::from_db(e)).collect();

        Ok(expenses)
    }

    pub async fn get_all_tags_by_user_id(&self, user_id: Uuid) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query_as!(
            Tag,
            "
            SELECT id, name 
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
            SELECT id, name 
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

mod schema {
    use uuid::Uuid;

    pub(super) struct ExpenseWithCategoryDb {
        pub id: Uuid,
        pub user_id: Uuid,
        pub category_id: Option<Uuid>,
        pub category_name: Option<String>,
        pub description: Option<String>,
        pub expense_date: chrono::NaiveDate,
        pub cost: rust_decimal::Decimal,
    }

    pub(super) struct TagDb {
        pub id: Uuid,
        pub name: String,
    }
}

impl FullExpense {
    fn from_db(expense: schema::ExpenseWithCategoryDb, tags: Vec<schema::TagDb>) -> Self {
        let category = expense
            .category_id
            .and_then(|id| expense.category_name.map(|name| Category { id, name }));

        FullExpense {
            id: expense.id,
            user_id: expense.user_id,
            description: expense.description,
            expense_date: expense.expense_date,
            cost: expense.cost,
            category: category,
            tags: tags
                .into_iter()
                .map(|t| Tag {
                    id: t.id,
                    name: t.name,
                })
                .collect(),
        }
    }
}

impl Expense {
    fn from_db(expense: schema::ExpenseWithCategoryDb) -> Self {
        let category = expense
            .category_id
            .and_then(|id| expense.category_name.map(|name| Category { id, name }));

        Expense {
            id: expense.id,
            user_id: expense.user_id,
            description: expense.description,
            expense_date: expense.expense_date,
            cost: expense.cost,
            category: category,
        }
    }
}
