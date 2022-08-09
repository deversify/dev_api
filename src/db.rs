use std::path::Path;

use crate::{ensure_env, Error};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySql, Pool, Postgres};

// This fn is the code we managed to generalize so far.
fn init() -> (String, bool) {
    tracing::info!("DB initializing...");

    let db_uri = ensure_env("DATABASE_URL");
    let migrate_db = ensure_env("MIGRATE_DB") == "ON";

    (db_uri, migrate_db)
}

// TODO: Generalize the code so that it works for any sqlx-supported DB.
async fn finish_pg(pool: &Pool<Postgres>, migrate_db: bool) {
    let _row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .expect("Failed while ensuring DB connection.");

    tracing::info!("DB connection ensured.");

    if migrate_db {
        tracing::info!("Migrations started...");
        sqlx::migrate::Migrator::new(Path::new("./migrations"))
            .await
            .expect("Migrator could not be created.")
            .run(pool)
            .await
            .map_err(|e| {
                tracing::error!("{}", e);
                e
            })
            .expect("Failed to run migrations!");
        tracing::info!("Migrated DB!");
    } else {
        tracing::info!("Migrations skipped.");
    }

    tracing::info!("DB initialized!");
}

// TODO: Generalize the code so that it works for any sqlx-supported DB.
#[tracing::instrument(name = "init_pg")]
pub async fn init_pg() -> Pool<Postgres> {
    let (db_uri, migrate_db) = init();

    let pool = PgPoolOptions::new().connect(&db_uri).await.unwrap();

    finish_pg(&pool, migrate_db).await;

    pool
}

pub async fn migrate_mysql(pool: &Pool<MySql>) -> crate::Result<()> {
    tracing::info!("Migrations started...");

    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Migrator could not be created.")
        .run(pool)
        .await?;

    tracing::info!("Migrated DB!");

    Ok(())
}

pub async fn migrate_postgres(pool: &Pool<Postgres>) -> crate::Result<()> {
    tracing::info!("Migrations started...");

    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Migrator could not be created.")
        .run(pool)
        .await?;

    tracing::info!("Migrated DB!");

    Ok(())
}

async fn finish_mysql(pool: &Pool<MySql>, migrate_db: bool) {
    let _row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .expect("Failed while ensuring DB connection.");

    tracing::info!("DB connection ensured.");

    if migrate_db {
        tracing::info!("Migrations started...");
        sqlx::migrate::Migrator::new(Path::new("./migrations"))
            .await
            .expect("Migrator could not be created.")
            .run(pool)
            .await
            .map_err(|e| {
                tracing::error!("{}", e);
                e
            })
            .expect("Failed to run migrations!");
        tracing::info!("Migrated DB!");
    } else {
        tracing::info!("Migrations skipped.");
    }

    tracing::info!("DB initialized!");
}

#[tracing::instrument(name = "init_mysql")]
pub async fn init_mysql() -> Pool<MySql> {
    let (db_uri, migrate_db) = init();

    let pool = MySqlPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to DB.");

    finish_mysql(&pool, migrate_db).await;
    pool
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("{:?}", error);
        Error::internal_error()
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        tracing::error!("{:?}", error);
        Error::internal_error()
    }
}
