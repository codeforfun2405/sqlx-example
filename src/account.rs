use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{errors::AppError, models::Account};

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub user_id: i64,
    pub balance: f64,
    pub currency: String,
    pub is_active: bool,
}

pub async fn create_account(input: &CreateAccount, pool: &PgPool) -> Result<Account, AppError> {
    let account = sqlx::query_as("INSERT INTO account(user_id, balance, currency, is_active) VALUES($1, $2, $3, $4) RETURNING account_id,user_id, balance, currency,created_at,updated_at,is_active")
        .bind(&input.user_id)
        .bind(&input.balance)
        .bind(&input.currency)
        .bind(&input.is_active)
        .fetch_one(pool)
        .await?;
    Ok(account)
}

pub async fn withdrawal(
    account_id: i64,
    amount: f64,
    tx: &mut Transaction<'static, Postgres>,
) -> Result<Account, AppError> {
    // 1 lock thw account for update

    let _ = sqlx::query("SELECT * FROM account WHERE account_id=$1 FOR UPDATE")
        .bind(account_id)
        .fetch_one(&mut **tx)
        .await?;

    // 2 update account balance
    let now: DateTime<Utc> = Utc::now();
    let account = sqlx::query_as(
        "UPDATE account SET balance=balance - $1, updated_at=$2 
            WHERE account_id=$3
            RETURNING account_id,user_id, balance, currency,created_at,updated_at,is_active",
    )
    .bind(amount)
    .bind(now)
    .bind(account_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(account)
}

pub async fn deposit<'a>(
    account_id: i64,
    amount: f64,
    tx: &mut Transaction<'static, Postgres>,
) -> Result<Account, AppError> {
    // 1 lock thw account for update

    let _ = sqlx::query("SELECT * FROM account WHERE account_id=$1 FOR UPDATE")
        .bind(account_id)
        .execute(&mut **tx)
        .await?;

    // 2 update account balance
    let now: DateTime<Utc> = Utc::now();
    let account = sqlx::query_as(
        "UPDATE account SET balance=balance + $1, updated_at=$2 
            WHERE account_id=$3
            RETURNING account_id,user_id, balance, currency,created_at,updated_at,is_active",
    )
    .bind(amount)
    .bind(now)
    .bind(account_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(account)
}
