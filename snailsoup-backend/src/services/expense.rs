use uuid::Uuid;

use crate::{
    db::{AppUserRepository, ExpenseRepository},
    domain::expense::{Category, Expense, FullExpense, Tag},
    utils::period::DatePeriod,
};
use std::sync::Arc;

pub struct ExpenseService {
    expense_repository: Arc<ExpenseRepository>,
    user_repository: Arc<AppUserRepository>,
}

pub enum ExpenseServiceGetError {
    InternalServerError,
}

pub enum ExpenseServiceCreateError {
    InternalServerError,
    NoUser,
    ValidationError(String),
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

    pub async fn get_expense(
        &self,
        expense_id: Uuid,
    ) -> Result<Option<FullExpense>, ExpenseServiceGetError> {
        Ok(self
            .expense_repository
            .get_expense(expense_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?)
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, ExpenseServiceGetError> {
        Ok(self
            .expense_repository
            .get_all_expenses()
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?)
    }

    pub async fn get_user_expenses(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Expense>>, ExpenseServiceGetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let expenses = self
            .expense_repository
            .get_all_expenses_by_user_id(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;

        Ok(Some(expenses))
    }

    pub async fn get_user_expenses_in_period(
        &self,
        user_id: Uuid,
        period: DatePeriod,
    ) -> Result<Option<Vec<Expense>>, ExpenseServiceGetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let expenses = self
            .expense_repository
            .get_all_expenses_by_user_id_in_period(user_id, period)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;

        Ok(Some(expenses))
    }

    pub async fn get_all_tags(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Tag>>, ExpenseServiceGetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let tags = self
            .expense_repository
            .get_all_tags_by_user_id(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;

        Ok(Some(tags))
    }

    pub async fn create_tag(
        &self,
        user_id: Uuid,
        name: &str,
    ) -> Result<Uuid, ExpenseServiceCreateError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceCreateError::InternalServerError)?;

        if user.is_none() {
            return Err(ExpenseServiceCreateError::NoUser);
        }

        let new_tag = Tag {
            id: Uuid::new_v4(),
            user_id: user_id,
            name: name.to_owned(),
        };

        self.expense_repository
            .insert_tag(new_tag)
            .await
            .map_err(|_| ExpenseServiceCreateError::InternalServerError)
    }

    pub async fn get_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Category>>, ExpenseServiceGetError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let tags = self
            .expense_repository
            .get_all_categories_by_user_id(user_id)
            .await
            .map_err(|_| ExpenseServiceGetError::InternalServerError)?;

        Ok(Some(tags))
    }

    pub async fn create_category(
        &self,
        user_id: Uuid,
        name: &str,
    ) -> Result<Uuid, ExpenseServiceCreateError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceCreateError::InternalServerError)?;

        if user.is_none() {
            return Err(ExpenseServiceCreateError::NoUser);
        }

        let new_category = Category {
            id: Uuid::new_v4(),
            user_id: user_id,
            name: name.to_owned(),
        };

        self.expense_repository
            .insert_category(new_category)
            .await
            .map_err(|_| ExpenseServiceCreateError::InternalServerError)
    }
}
