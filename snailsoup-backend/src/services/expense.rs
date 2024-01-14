use uuid::Uuid;

use crate::{
    db::{AppUserRepository, ExpenseRepository},
    domain::expense::{Category, Expense, FullExpense, Tag},
};
use std::sync::Arc;

pub struct ExpenseService {
    expense_repository: Arc<ExpenseRepository>,
    user_repository: Arc<AppUserRepository>,
}

pub enum ExpenseServiceError {
    InternalServerError,
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
    ) -> Result<Option<FullExpense>, ExpenseServiceError> {
        Ok(self
            .expense_repository
            .get_expense(expense_id)
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?)
    }

    pub async fn get_all_expenses(&self) -> Result<Vec<Expense>, ExpenseServiceError> {
        Ok(self
            .expense_repository
            .get_all_expenses()
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?)
    }

    pub async fn get_all_tags(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Tag>>, ExpenseServiceError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let tags = self
            .expense_repository
            .get_all_tags_by_user_id(user_id)
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?;

        Ok(Some(tags))
    }

    pub async fn get_all_categories(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Vec<Category>>, ExpenseServiceError> {
        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?;
        if user.is_none() {
            return Ok(None);
        }

        let tags = self
            .expense_repository
            .get_all_categories_by_user_id(user_id)
            .await
            .map_err(|_| ExpenseServiceError::InternalServerError)?;

        Ok(Some(tags))
    }
}
