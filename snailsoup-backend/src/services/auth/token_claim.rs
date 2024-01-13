use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: String,
    pub created_at: i64,
    pub exp: i64,
}
