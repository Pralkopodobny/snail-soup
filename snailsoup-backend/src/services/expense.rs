use crate::db::ExpenseRepository;
use crate::domain::Expense;
use std::sync::Arc;

pub struct ExpenseService {
    expense_repository: Arc<ExpenseRepository>,
}

impl ExpenseService {
    pub fn new(expense_repository: Arc<ExpenseRepository>) -> ExpenseService {
        ExpenseService {
            expense_repository: expense_repository,
        }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Option<Expense> {
        match self.expense_repository.get(id).await {
            Ok(expense) => expense,
            Err(_) => None,
        }
    }

    pub async fn get_all(&self) -> Vec<Expense> {
        match self.expense_repository.get_all().await {
            Ok(expenses) => expenses,
            Err(_) => Vec::new(),
        }
    }
}
