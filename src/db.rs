use std::path::Path;

use crate::ensure_env;
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, MySql, Pool, Postgres};

fn init() -> (String, bool) {
    tracing::info!("DB initializing...");

    let db_uri = ensure_env("DATABASE_URL");
    let migrate_db = ensure_env("MIGRATE_DB") == "ON";

    (db_uri, migrate_db)
}

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

pub async fn init_pg() -> Pool<Postgres> {
    let (db_uri, migrate_db) = init();

    let pool = PgPoolOptions::new().connect(&db_uri).await.unwrap();

    finish_pg(&pool, migrate_db).await;

    pool
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

pub async fn init_mysql() -> Pool<MySql> {
    let (db_uri, migrate_db) = init();

    let pool = MySqlPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to DB.");

    finish_mysql(&pool, migrate_db).await;
    pool
}
