// Identity domain and authentication use-cases live here.

use argon2::Argon2;
use argon2::password_hash::{
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub email: String,
    pub id: uuid::Uuid,
}

pub async fn register(
    pool: &sqlx::PgPool,
    request: RegisterRequest,
) -> Result<RegisterResponse, IdentityError> {
    let hashed_password = hash_password(&request.password).unwrap();

    let mut tx = pool.begin().await?;
    let user_id = uuid::Uuid::new_v4();

    sqlx::query(r#" INSERT INTO users (id,email, password_hash) VALUES ($1,$2,$3)"#)
        .bind(user_id)
        .bind(&request.email)
        .bind(hashed_password)
        .execute(&mut *tx)
        .await?;

    //Used query Scaler for performance
    //Instead of fetching the whole db role, we get value from the first column
    let customer_role_id: uuid::Uuid = sqlx::query_scalar(
        r#"
    SELECT id
    FROM roles
    WHERE code = 'customer'
    "#,
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
    INSERT INTO user_roles (user_id, role_id)
    VALUES ($1, $2)
    "#,
    )
    .bind(user_id)
    .bind(customer_role_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(RegisterResponse {
        id: user_id,
        email: request.email,
    })
    // todo!()
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password)
}

fn verify_password(
    password: &str,
    hashed_password: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hashed_password)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("password hashing failed: {0}")]
    PasswordHash(String),

    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_password() {
        let first = hash_password("secret123").unwrap();
        let second = hash_password("secret123").unwrap();

        assert!(first.starts_with("$argon2id$"));
        assert_ne!(first, "secret123");
        assert_ne!(first, second);
    }

    #[test]
    fn verifies_password() {
        let hash = hash_password("secret123").unwrap();

        assert!(verify_password("secret123", &hash).unwrap());
        assert!(!verify_password("wrong-password", &hash).unwrap());
    }
}
