mod token_claim;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{db::AppUserRepository, domain::AppUser};
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
    UnexpectedError,
}

pub enum AuthError {
    InvalidToken,
    ExpiredToken,
    UserDoesNotExist,
    InternalError,
}

impl AuthService {
    pub fn new(user_repository: Arc<AppUserRepository>) -> AuthService {
        AuthService { user_repository }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, LoginError> {
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

        let now = Utc::now();

        let claims = TokenClaims {
            id: user.id.to_string(),
            created_at: now.timestamp(),
            exp: (now + Duration::minutes(60)).timestamp(), //TODO: use time from .env file
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("TOP SECRET".as_ref()), //TODO: use secret from .env file
        )
        .map_err(|_| LoginError::UnexpectedError)?;

        Ok(token)
    }

    pub async fn auth_bearer_token(&self, token: &str) -> Result<AppUser, AuthError> {
        let claims = decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret("TOP SECRET".as_ref()), //TODO: use secret from .env file
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        let user_id = Uuid::parse_str(&claims.claims.id).map_err(|_| AuthError::InvalidToken)?;

        let now = Utc::now();
        let expire_date = match DateTime::<Utc>::from_timestamp(claims.claims.exp, 0) {
            Some(t) => t,
            None => Err(AuthError::InvalidToken)?,
        };

        if expire_date < now {
            return Err(AuthError::ExpiredToken);
        }

        let user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|_| AuthError::InternalError)?;
        match user {
            Some(user) => Ok(user),
            None => Err(AuthError::UserDoesNotExist),
        }
    }
}
