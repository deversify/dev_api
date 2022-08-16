use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Clone, Debug)]
pub struct Migration {
    pub version: i64,
    pub description: String,
    pub installed_on: chrono::DateTime<chrono::Utc>,
    pub success: i8,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}
