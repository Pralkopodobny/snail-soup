use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: String,
    pub created_at: usize,
    pub exp: usize,
}