use crate::ensure_env;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn init_mysql() -> Pool<MySql> {
    tracing::info!("DB initializing...");

    let db_uri = ensure_env("DATABASE_URL");
    let _migrate_db = ensure_env("MIGRATE_DB") == "ON";

    let pool = MySqlPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to DB.");

    let _row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed while ensuring DB connection.");

    tracing::info!("DB connection ensured.");

    // if migrate_db {
    //     tracing::info!("Migrations started...");
    //     sqlx::migrate!()
    //         .run(&pool)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("{}", e);
    //             e
    //         })
    //         .expect("Failed to run migrations!");
    //     tracing::info!("Migrated DB!");
    // } else {
    //     tracing::info!("Migrations skipped.");
    // }

    tracing::info!("DB initialized!");
    pool
}
