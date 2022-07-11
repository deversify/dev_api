use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
}
