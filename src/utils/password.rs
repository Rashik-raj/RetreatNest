use password_worker::{Argon2idConfig, PasswordWorker};

use crate::env::ENV;

const MAX_TRHEADS: usize = 8;

pub async fn create_password(password: &str) -> Result<String, Box<dyn std::error::Error>>{
    let salt : Vec<u8>= ENV.password_salt.clone().into();
    let password_worker = PasswordWorker::new_argon2id(MAX_TRHEADS)?;
    let hashed_password = password_worker
        .hash(
            password,
            Argon2idConfig {
                salt,
                ..Default::default()
            },
        )
        .await?;
    Ok(hashed_password)
}

pub async fn check_password(password: &str, hashed_password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let password_worker = PasswordWorker::new_argon2id(MAX_TRHEADS)?;
    let is_valid: bool = password_worker.verify(password, hashed_password).await?;
    Ok(is_valid)
}