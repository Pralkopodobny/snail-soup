use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::AppUser;

#[derive(Clone)]
pub struct AppUserRepository {
    pool: Pool<Postgres>,
}

impl AppUserRepository {
    pub fn new(pool: Pool<Postgres>) -> AppUserRepository {
        AppUserRepository { pool: pool }
    }

    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<AppUser>, sqlx::Error> {
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
