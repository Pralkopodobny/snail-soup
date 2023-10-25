use sqlx::Error;
use sqlx::Postgres;
use sqlx::Pool;

use async_trait::async_trait;

use crate::domain::AppUser;

#[async_trait]
pub trait AppUserRepository {
    async fn get(&self, id: uuid::Uuid) -> Result<Option<AppUser>, Error>;
}

pub struct MyAppUserRepository {
    pub pool : Pool<Postgres>
}

#[async_trait]
impl AppUserRepository for MyAppUserRepository {
    async fn get(&self, id: uuid::Uuid) -> Result<Option<AppUser>, Error> {
        let user = sqlx::query_as!(AppUser,
            "
            SELECT * FROM app_users WHERE id = $1
            ", id
        ).fetch_optional(&self.pool).await?;
        Ok(user)
    }
}