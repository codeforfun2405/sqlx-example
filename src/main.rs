use sqlx::PgPool;
use sqlx_example::{
    account::{self, CreateAccount},
    errors::AppError,
    models::{Account, Todo, User},
    todolist::{self, CreateTodo, GetTodosArgs},
    user::{self, CreateUser},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let pool = PgPool::connect("postgres://db_manager:xxx@localhost:5432/my_todo_list").await?;
    println!("connected database: {:?}", pool);
    println!();

    // let user1 = create_user(&pool, "Alex".to_string()).await?;
    // let user2 = create_user(&pool, "Bob".to_string()).await?;

    let account1 = create_account(&pool, 2 as i64, 1000.0 as f64).await?;
    let account2 = create_account(&pool, 3 as i64, 1000.0 as f64).await?;

    println!(
        "transfer from account1: {:?}, to account2: {:?}",
        account1, account2
    );
    transfer_from_to(
        &pool,
        account1.account_id,
        account2.account_id,
        100.0 as f64,
    )
    .await?;

    Ok(())
}

async fn create_user(pool: &PgPool, username: String) -> Result<User, AppError> {
    let user = CreateUser {
        username: username.clone(),
        email: format!("{}@acme.org", &username.to_lowercase()),
    };

    user::create_user(&user, pool).await
}

async fn create_todo(pool: &PgPool, user_id: i64) -> Result<Todo, AppError> {
    let create_todo = CreateTodo {
        user_id,
        title: "Query Data with SQLX".to_string(),
        description: "sqlx usage".to_string(),
        status: 0,
    };

    todolist::create_todo(&create_todo, pool).await
}

async fn get_todo_by_user_id(pool: &PgPool, user_id: i64) -> Result<Vec<Todo>, AppError> {
    todolist::get_todos_by_user_id(
        GetTodosArgs {
            user_id,
            offset: 0,
            limit: 100,
            status: None,
        },
        pool,
    )
    .await
}

async fn create_account(pool: &PgPool, user_id: i64, balance: f64) -> Result<Account, AppError> {
    let create_account = CreateAccount {
        user_id,
        balance,
        currency: "USD".to_string(),
        is_active: true,
    };

    account::create_account(&create_account, pool).await
}

async fn transfer_from_to(
    pool: &PgPool,
    from_user: i64,
    to_user: i64,
    amount: f64,
) -> Result<(), AppError> {
    println!(
        "start transaction transafer from: {} to: {}, with amount: {}",
        from_user, to_user, amount
    );

    let mut tx = pool.begin().await?;

    let from_account = account::withdrawal(from_user, amount, &mut tx).await?;
    println!("withdrawal {} from acount: {:?}", amount, from_account);

    let to_account = account::deposit(to_user, amount, &mut tx).await?;
    println!("deposit {} from acount: {:?}", amount, to_account);

    match tx.commit().await {
        Ok(()) => {
            println!("success transfer {} to {}", amount, to_account.account_id);
        }
        Err(e) => {
            println!(
                "transfer {} to {} failed: {:?}",
                amount, to_account.account_id, e
            );
        }
    }

    Ok(())
}
