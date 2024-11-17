use serde::Deserialize;
use sqlx::PgPool;

use crate::{errors::AppError, models::User};

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

pub async fn create_user(input: &CreateUser, pool: &PgPool) -> Result<User, AppError> {
    let user_result = get_user_by_email(&input.email, pool).await?;
    if user_result.is_some() {
        return Err(AppError::EmailAlreadyExists(format!(
            "{}",
            input.email.clone()
        )));
    }

    let user = sqlx::query_as("INSERT INTO users(username, email) VALUES($1, $2) RETURNING id,username,email,created_at,updated_at")
        .bind(&input.username)
        .bind(&input.email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, AppError> {
    let user =
        sqlx::query_as("SELECT id,username,email,created_at,updated_at FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}

pub async fn get_user_by_id(id: i64, pool: &PgPool) -> Result<Option<User>, AppError> {
    let user =
        sqlx::query_as("SELECT id,username,email,created_at,updated_at FROM users WHERE id=$1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(user)
}
