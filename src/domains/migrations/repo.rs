use sqlx::{MySql, Pool, Postgres};

use crate::db::{migrate_mysql, migrate_postgres};

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
}
