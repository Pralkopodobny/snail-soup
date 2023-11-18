use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::AppUser;

pub struct AppUserRepository<'a> {
    pub pool: &'a Pool<Postgres>,
}

impl<'a> AppUserRepository<'a> {
    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<AppUser>, Error> {
        let user = sqlx::query_as!(
            AppUser,
            "
            SELECT * FROM app_users WHERE id = $1
            ",
            id
        )
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }
}
