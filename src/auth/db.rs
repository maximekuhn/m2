use sqlx::{Sqlite, Transaction};

use crate::{auth::model::Entry, error::AppError};

/// WARNING: does not insert [Entry::sessions].
pub async fn insert(tx: &'_ mut Transaction<'_, Sqlite>, entry: Entry) -> Result<(), AppError> {
    sqlx::query(
        r#"
    INSERT INTO auth_entry
    (id, user_id, hashed_password, created_at) VALUES
    (?, ?, ?, ?)
    "#,
    )
    .bind(entry.id)
    .bind(entry.user_id.value())
    .bind(entry.hashed_password)
    .bind(entry.created_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}
