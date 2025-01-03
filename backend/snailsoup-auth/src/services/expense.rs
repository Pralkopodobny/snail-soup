use uuid::Uuid;

use crate::{
    db::{AppUserRepository, ExpenseRepository},
    domain::expense::{
        Category, CategoryData, Expense, FullExpense, FullExpenseData, Tag, TagData,
    },
    utils::period::DatePeriod,
};
use std::sync::Arc;

pub enum GetError {
    Internal,
}

pub enum CreateError {
    Internal,
    NoUser,
    Validation(String),
}

pub struct ExpenseService {
    expense_repository: Arc<ExpenseRepository>,
    user_repository: Arc<AppUserRepository>,
}

impl ExpenseService {
    pub fn new(
        expense_repository: Arc<ExpenseRepository>,
        user_repository: Arc<AppUserRepository>,
    ) -> ExpenseService {
        ExpenseService {
            expense_repository: expense_repository,
            user_repository: user_repository,
        }
    }

    pub async fn get_expense(&self, expense_id: Uuid) -> Result<Option<FullExpense>, GetError> {
        Ok(self
            .expense_repository
            .get_expense(expense_id)
            .await
            .map_err(|_| GetError::Internal)?)
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, GetError> {
        Ok(self
            .expense_repository
            .get_all_expenses()
            .await
            .map_err(|_| GetError::Internal)?)
    }

    pub async fn get_expenses_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Expense>>, GetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| GetError::Internal)?;
        if user.is_none() {
            return Ok(None);
        }

        let expenses = self
            .expense_repository
            .get_all_expenses_by_user_id(user_id)
            .await
            .map_err(|_| GetError::Internal)?;

        Ok(Some(expenses))
    }

    pub async fn get_expenses_for_user_in_period(
        &self,
        user_id: Uuid,
        period: DatePeriod,
    ) -> Result<Option<Vec<Expense>>, GetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| GetError::Internal)?;
        if user.is_none() {
            return Ok(None);
        }

        let expenses = self
            .expense_repository
            .get_all_expenses_by_user_id_in_period(user_id, period)
            .await
            .map_err(|_| GetError::Internal)?;

        Ok(Some(expenses))
    }

    pub async fn create_expense(&self, full_expense: FullExpenseData) -> Result<Uuid, CreateError> {
        let user = self
            .user_repository
            .get(full_expense.expense.user_id)
            .await
            .map_err(|_| CreateError::NoUser)?;

        if user.is_none() {
            return Err(CreateError::NoUser);
        }

        if !full_expense.tags_ids.is_empty() {
            let user_tags: Vec<Uuid> = self
                .expense_repository
                .get_all_tags_by_user_id(full_expense.expense.user_id)
                .await
                .map_err(|_| CreateError::Internal)?
                .into_iter()
                .map(|user_tag| user_tag.id)
                .collect();

            if !full_expense
                .tags_ids
                .iter()
                .all(|tag| user_tags.contains(tag))
            {
                return Err(CreateError::Validation("Invalid tags list".to_owned()));
            }
        }

        let new_expense = FullExpense {
            id: Uuid::new_v4(),
            data: full_expense,
        };

        self.expense_repository
            .insert_full_expense(new_expense)
            .await
            .map_err(|_| CreateError::Internal)
    }

    pub async fn get_tags_for_user(&self, user_id: Uuid) -> Result<Option<Vec<Tag>>, GetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| GetError::Internal)?;
        if user.is_none() {
            return Ok(None);
        }

        let tags = self
            .expense_repository
            .get_all_tags_by_user_id(user_id)
            .await
            .map_err(|_| GetError::Internal)?;

        Ok(Some(tags))
    }

    pub async fn get_tag(&self, tag_id: Uuid) -> Result<Option<Tag>, GetError> {
        self.expense_repository
            .get_tag(tag_id)
            .await
            .map_err(|_| GetError::Internal)
    }

    pub async fn create_tag(&self, tag: TagData) -> Result<Uuid, CreateError> {
        let user = self
            .user_repository
            .get(tag.user_id)
            .await
            .map_err(|_| CreateError::Internal)?;

        if user.is_none() {
            return Err(CreateError::NoUser);
        }

        let new_tag = Tag {
            id: Uuid::new_v4(),
            data: tag,
        };

        self.expense_repository
            .insert_tag(new_tag)
            .await
            .map_err(|_| CreateError::Internal)
    }

    pub async fn update_tag(&self, tag: Tag) -> Result<Option<Uuid>, GetError> {
        self.expense_repository
            .update_tag(tag)
            .await
            .map_err(|_| GetError::Internal)
    }

    pub async fn get_categories_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Category>>, GetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| GetError::Internal)?;
        if user.is_none() {
            return Ok(None);
        }

        let categories = self
            .expense_repository
            .get_all_categories_by_user_id(user_id)
            .await
            .map_err(|_| GetError::Internal)?;

        Ok(Some(categories))
    }

    pub async fn get_category(&self, category_id: Uuid) -> Result<Option<Category>, GetError> {
        self.expense_repository
            .get_category(category_id)
            .await
            .map_err(|_| GetError::Internal)
    }

    pub async fn create_category(&self, category: CategoryData) -> Result<Uuid, CreateError> {
        let user = self
            .user_repository
            .get(category.user_id)
            .await
            .map_err(|_| CreateError::Internal)?;

        if user.is_none() {
            return Err(CreateError::NoUser);
        }

        let new_category = Category {
            id: Uuid::new_v4(),
            data: category,
        };

        self.expense_repository
            .insert_category(new_category)
            .await
            .map_err(|_| CreateError::Internal)
    }

    pub async fn update_category(&self, category: Category) -> Result<Option<Uuid>, GetError> {
        self.expense_repository
            .update_category(category)
            .await
            .map_err(|_| GetError::Internal)
    }
}
