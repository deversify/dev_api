use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tokens<'a> {
    pub access_token: String,
    pub token_type: &'a str,
    pub expires_in: i64,
    pub refresh_token: String,
}
