use sqlx::{MySql, Pool};

use crate::db::migrate_mysql;

#[derive(Clone, Debug)]
pub struct RepoImpl {
    pub pool: Pool<MySql>,
}

impl RepoImpl {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    pub async fn run(&self) -> crate::Result<()> {
        migrate_mysql(&self.pool).await
    }
}
