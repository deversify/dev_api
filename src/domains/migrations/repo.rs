use super::types::Migration;
use crate::db::{migrate_mysql, migrate_postgres};
use sqlx::{query_as, MySql, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct RepoMySqlImpl {
    pub pool: Pool<MySql>,
}

impl RepoMySqlImpl {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn run(&self) -> crate::Result<()> {
        migrate_mysql(&self.pool).await
    }

    pub async fn list(&self) -> crate::Result<Vec<Migration>> {
        let result = query_as::<_,Migration>(
            "SELECT version, description, installed_on, success, checksum, execution_time FROM _sqlx_migrations ORDER BY version DESC"
        ).fetch_all(&self.pool).await?;

        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct RepoPostgresImpl {
    pub pool: Pool<Postgres>,
}

impl RepoPostgresImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn run(&self) -> crate::Result<()> {
        migrate_postgres(&self.pool).await
    }

    pub async fn list(&self) -> crate::Result<Vec<Migration>> {
        let result = query_as::<_,Migration>(
            "SELECT version, description, installed_on, success, checksum, execution_time FROM _sqlx_migrations ORDER BY version DESC"
        ).fetch_all(&self.pool).await?;

        Ok(result)
    }
}
