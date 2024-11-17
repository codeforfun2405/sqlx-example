use serde::Deserialize;
use sqlx::PgPool;

use crate::{errors::AppError, models::Todo};

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub user_id: i64,
    pub title: String,
    pub description: String,
    pub status: i16,
}

#[derive(Debug)]
pub struct GetTodosArgs {
    pub user_id: i64,
    pub offset: i64,
    pub limit: i64,
    pub status: Option<i16>,
}

pub async fn create_todo(input: &CreateTodo, pool: &PgPool) -> Result<Todo, AppError> {
    let todo = sqlx::query_as(
        r#"INSERT INTO todos (user_id,title,description,status)
                VALUES ($1,$2,$3,$4)
                RETURNING id,user_id,title,description,status,created_at,updated_at;
                "#,
    )
    .bind(input.user_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.status)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn get_todos_by_user_id(
    args: GetTodosArgs,
    pool: &PgPool,
) -> Result<Vec<Todo>, AppError> {
    let mut offset = args.offset;
    if args.offset < 0 {
        offset = 0;
    }

    let mut limit = args.limit;
    if args.limit <= 0 || args.limit > 100 {
        limit = 100
    }

    let mut status = 0 as i16;
    if args.status.is_some() {
        status = args.status.unwrap();
    }

    let todos = sqlx::query_as(
        r#"
        SELECT id,user_id,title,description,status,created_at,updated_at
        FROM todos
        WHERE user_id=$1 AND status=$2
        ORDER BY id DESC
        LIMIT $3
        OFFSET $4;
    "#,
    )
    .bind(args.user_id)
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(todos)
}

pub async fn update_todo_status(id: i64, status: i16, pool: &PgPool) -> Result<Todo, AppError> {
    let todo = sqlx::query_as(
        r#"
        UPDATE todos SET status=$1 WHERE id=$2
        RETURNING id,user_id,title,description,status,created_at,updated_at;
    "#,
    )
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}
