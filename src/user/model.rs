use derive_new::new;
use email_address::EmailAddress;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::AppError;

#[derive(new)]
pub struct User {
    pub id: UserId,
    pub name: Username,
    pub email: EmailAddress,
    pub roles: Vec<Role>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub struct UserId {
    val: Uuid,
}

impl UserId {
    const FIELD: &str = "userId";

    pub fn value(&self) -> Uuid {
        self.val
    }
}

impl FromStr for UserId {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: Uuid = s.parse().map_err(|_err| AppError::InvalidInput {
            field: Self::FIELD,
            reason: Some("invalid UUID"),
        })?;
        Ok(Self { val: id })
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self { val: value }
    }
}

pub struct Username {
    val: String,
}

impl Username {
    const FIELD: &str = "username";

    pub fn value(&self) -> String {
        self.val.clone()
    }
}

impl FromStr for Username {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("empty"),
            });
        }
        if s.len() < 3 {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("too small (min 3 chars)"),
            });
        }
        if s.len() > 32 {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("too big (max 32 chars)"),
            });
        }
        if !s
            .chars()
            .filter(|c| !c.is_alphanumeric() && *c != '-' && *c != '_')
            .collect::<Vec<_>>()
            .is_empty()
        {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("unallowed chars (allowed: alphanumeric, - and _)"),
            });
        }
        let first: char = s.chars().next().expect("min len is 3");
        if !first.is_alphabetic() {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("must start with a letter"),
            });
        }

        Ok(Self { val: s.to_string() })
    }
}

pub enum Role {
    User,
    Admin,
}
