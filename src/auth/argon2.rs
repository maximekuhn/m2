use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{auth::model::Password, error::AppError};

pub fn hash_password(p: Password) -> Result<Vec<u8>, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(p.value().as_bytes(), &salt)?
        .to_string()
        .as_bytes()
        .to_vec())
}
