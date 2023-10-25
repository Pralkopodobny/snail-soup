pub struct AppUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub account_role: String
}