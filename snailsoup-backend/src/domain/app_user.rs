use uuid::Uuid;

#[derive(Clone)]
pub struct AppUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub account_role: String,
}
