use sqlx::PgPool;
use tracing::info;
use crate::models::users::{User, DbUser, CreateUserRequest};
use sqlx::types::chrono::{DateTime, Utc};

pub async fn create_user(
    pool: &PgPool,
    user: &CreateUserRequest,
    password_hash: String,
) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as!(
        DbUser,
        r#"
        INSERT INTO users (username, password_hash, telegram_id, created_at, is_active, is_admin)
        VALUES ($1, $2, $3, NOW(), true, $4)
        RETURNING id, username, password_hash, telegram_id, created_at, is_active, last_login, is_admin
        "#,
        user.username,
        password_hash,
        user.telegram_id,
        user.is_admin.unwrap_or(false)
    )
    .fetch_one(pool)
    .await
}

pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as!(
        DbUser,
        r#"
        SELECT id, username, password_hash, telegram_id, created_at, is_active, last_login, is_admin
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await
}

pub async fn update_last_login(
    pool: &PgPool,
    user_id: i32,
    last_login: DateTime<Utc>,
) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as!(
        DbUser,
        r#"
        UPDATE users
        SET last_login = $1
        WHERE id = $2
        RETURNING id, username, password_hash, telegram_id, created_at, is_active, last_login, is_admin
        "#,
        last_login,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn deactivate_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<DbUser, sqlx::Error> {
    sqlx::query_as!(
        DbUser,
        r#"
        UPDATE users
        SET is_active = false
        WHERE id = $1
        RETURNING id, username, password_hash, telegram_id, created_at, is_active, last_login, is_admin
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<DbUser>, sqlx::Error> {
    sqlx::query_as!(
        DbUser,
        r#"
        SELECT id, username, password_hash, telegram_id, created_at, is_active, last_login, is_admin
        FROM users
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await
}