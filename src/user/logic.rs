use chrono::Utc;
use email_address::EmailAddress;
use uuid::Uuid;

use crate::{
    error::AppError,
    user::{
        db,
        model::{Role, User, UserId, Username},
    },
};

pub async fn create(
    tx: &'_ mut sqlx::Transaction<'_, sqlx::Sqlite>,
    name: Username,
    email: EmailAddress,
) -> Result<UserId, AppError> {
    if db::exists_by_name(tx, &name).await? {
        return Err(AppError::UsernameAlreadyTaken);
    }
    if db::exists_by_email(tx, &email).await? {
        return Err(AppError::EmailAlreadyTaken);
    }
    let id = Uuid::now_v7().into();
    let user = User::new(id, name, email, vec![Role::User], Utc::now());
    db::insert(tx, user).await?;
    Ok(id)
}
