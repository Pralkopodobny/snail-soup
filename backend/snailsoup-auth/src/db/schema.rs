use uuid::Uuid;

use crate::domain::app_user::AppUser;

pub struct AppUserSchema {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub account_role: String,
}

impl Into<AppUser> for AppUserSchema {
    fn into(self) -> AppUser {
        AppUser {
            id: self.id,
            username: self.username,
            password_hash: self.password_hash,
            account_role: self.account_role,
        }
    }
}
