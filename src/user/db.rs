use email_address::EmailAddress;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

use crate::{
    error::AppError,
    user::model::{Role, User, Username},
};

pub async fn insert(tx: &'_ mut Transaction<'_, Sqlite>, user: User) -> Result<(), AppError> {
    sqlx::query(
        r#"
    INSERT INTO user
    (id, name, email, created_at) VALUES
    (?, ?, ?, ?)
    "#,
    )
    .bind(user.id.value())
    .bind(user.name.value())
    .bind(user.email.email())
    .bind(user.created_at)
    .execute(tx.as_mut())
    .await?;

    for role in user.roles {
        sqlx::query(
            r#"
        INSERT INTO user_role
        (user_id, role) VALUES
        (?, ?)
        "#,
        )
        .bind(user.id.value())
        .bind(role_to_db(role))
        .execute(tx.as_mut())
        .await?;
    }
    Ok(())
}

pub async fn exists_by_name(
    tx: &'_ mut Transaction<'_, Sqlite>,
    name: &Username,
) -> Result<bool, AppError> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM user
        WHERE name = ?
        "#,
    )
    .bind(name.value())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}

pub async fn exists_by_email(
    tx: &'_ mut Transaction<'_, Sqlite>,
    email: &EmailAddress,
) -> Result<bool, AppError> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM user
        WHERE email = ?
        "#,
    )
    .bind(email.email())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}

fn role_to_db(r: Role) -> u8 {
    match r {
        Role::User => 10,
        Role::Admin => 20,
    }
}
