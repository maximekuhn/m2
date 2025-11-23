use chrono::Utc;
use uuid::Uuid;

use crate::{
    auth::{
        argon2::hash_password,
        db,
        model::{Entry, Password},
    },
    error::AppError,
    user::model::UserId,
};

pub async fn create_entry(
    tx: &'_ mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: UserId,
    password: Password,
) -> Result<(), AppError> {
    let hashed_password = hash_password(password)?;
    let entry = Entry::new(Uuid::now_v7(), user_id, hashed_password, Utc::now(), vec![]);
    db::insert(tx, entry).await?;
    Ok(())
}
