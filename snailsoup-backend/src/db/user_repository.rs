use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

use crate::domain::AppUser;

#[derive(Clone)]
pub struct AppUserRepository {
    pool: Pool<Postgres>,
}

impl AppUserRepository {
    pub fn new(pool: Pool<Postgres>) -> AppUserRepository {
        AppUserRepository { pool: pool }
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<AppUser>, sqlx::Error> {
        let user = sqlx::query_as!(
            AppUser,
            "
            SELECT * FROM app_users WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn insert(&self, user: AppUser) -> Result<AppUser, sqlx::Error> {
        let created_user = sqlx::query_as!(AppUser,
        "
        INSERT INTO app_users(id, username, password_hash, account_role) VALUES ($1, $2, $3, $4) RETURNING id, username, password_hash, account_role
        ",
        user.id,
        user.username,
        user.password_hash,
        user.account_role
        ).fetch_one(&self.pool)
        .await?;
        Ok(created_user)
    }

    pub async fn get_by_name(&self, username: &str) -> Result<Option<AppUser>, sqlx::Error> {
        let user = sqlx::query_as!(
            AppUser,
            "
            SELECT * FROM app_users WHERE username = $1
            ",
            username
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_all(&self) -> Result<Vec<AppUser>, sqlx::Error> {
        let users = sqlx::query_as!(
            AppUser,
            "
            SELECT * FROM app_users
            "
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }
}
