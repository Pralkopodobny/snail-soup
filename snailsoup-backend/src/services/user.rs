use uuid::Uuid;

use crate::db::AppUserRepository;
use crate::domain::AppUser;
use std::sync::Arc;

pub struct UserService {
    user_repository: Arc<AppUserRepository>,
}

impl UserService {
    pub fn new(expense_repository: Arc<AppUserRepository>) -> UserService {
        UserService {
            user_repository: expense_repository,
        }
    }

    pub async fn get(&self, id: Uuid) -> Option<AppUser> {
        match self.user_repository.get(id).await {
            Ok(expense) => expense,
            Err(_) => None,
        }
    }

    pub async fn get_all(&self) -> Vec<AppUser> {
        match self.user_repository.get_all().await {
            Ok(expenses) => expenses,
            Err(_) => Vec::new(),
        }
    }
}
