use std::str::FromStr;

use sqlx::SqlitePool;

pub async fn setup_database(db_file: &str) -> sqlx::Result<sqlx::SqlitePool> {
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(db_file)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = sqlx::SqlitePool::connect_with(options).await?;
    apply_migrations(&pool).await?;
    Ok(pool)
}

pub async fn apply_migrations(pool: &SqlitePool) -> sqlx::Result<()> {
    Ok(sqlx::migrate!().run(pool).await?)
}
