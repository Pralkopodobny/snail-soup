use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use crate::domain::AppUser;

#[derive(Clone)]
pub struct AppUserRepository {
    pub pool: Pool<Postgres>,
}

impl AppUserRepository {
    pub async fn get(&self, id: uuid::Uuid) -> Result<Option<AppUser>, Error> {
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
}
