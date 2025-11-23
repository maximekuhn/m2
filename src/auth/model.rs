use std::str::FromStr;

use chrono::{DateTime, Utc};
use derive_new::new;
use uuid::Uuid;

use crate::{error::AppError, user::model::UserId};

#[derive(new)]
pub struct Entry {
    pub id: Uuid,
    pub user_id: UserId,
    pub hashed_password: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub sessions: Vec<Session>,
}

pub struct Session {
    pub id: [u8; 128],
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub struct Password {
    val: String,
}

impl Password {
    const FIELD: &str = "password";

    pub fn value(&self) -> String {
        self.val.clone()
    }
}

impl FromStr for Password {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("empty"),
            });
        }

        if s.len() < 12 {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("too small (min 12 chars)"),
            });
        }

        if s.len() > 72 {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("too big (max 72 chars)"),
            });
        }

        let has_uppercase = s.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = s.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = s.chars().any(|c| c.is_ascii_digit());
        let has_special = s.chars().any(|c| !c.is_ascii_alphanumeric());

        if !(has_uppercase && has_lowercase && has_digit && has_special) {
            return Err(AppError::InvalidInput {
                field: Self::FIELD,
                reason: Some("not strong enough"),
            });
        }

        Ok(Self { val: s.to_string() })
    }
}
