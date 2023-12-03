mod token_claim;

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::db::AppUserRepository;
use std::sync::Arc;

pub use self::token_claim::TokenClaims;

pub struct AuthService {
    user_repository: Arc<AppUserRepository>,
}

pub enum LoginError {
    IncorrectUser,
    IncorrectPassword,
    InternalError,
    InternalPasswordError,
}

impl AuthService {
    pub fn new(expense_repository: Arc<AppUserRepository>) -> AuthService {
        AuthService {
            user_repository: expense_repository,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<TokenClaims, LoginError> {
        let user_opt = match self.user_repository.get_by_name(username).await {
            Ok(user) => user,
            Err(_) => Err(LoginError::InternalError)?,
        };

        let user = match user_opt {
            Some(user) => user,
            None => Err(LoginError::IncorrectUser)?,
        };

        let is_valid = match PasswordHash::new(&user.password_hash) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true),
            Err(_) => {
                println!("{} has incorrect password in database!", user.username);
                Err(LoginError::InternalPasswordError)?
            }
        };

        if !is_valid {
            return Err(LoginError::IncorrectPassword);
        }

        let now = chrono::Utc::now();
        //TODO: encode using secret and use time from .env file
        Ok(TokenClaims {
            id: user.id.to_string(),
            created_at: now.timestamp() as usize,
            exp: (now + chrono::Duration::minutes(60)).timestamp() as usize,
        })
    }
}
