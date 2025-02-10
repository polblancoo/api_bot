use crate::models::users::CreateUserRequest;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn create_user(
    pool: &PgPool,
    req: &CreateUserRequest,
) -> Result<User, sqlx::Error> {
    let now = Utc::now();
    let password_hash = hash(req.password.as_bytes(), DEFAULT_COST).unwrap();

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        req.username,
        req.email,
        password_hash,
        now as _,
        now as _,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_user(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users 
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn authenticate_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let user = get_by_username(pool, username).await?;

    if let Some(user) = user {
        if verify(password.as_bytes(), &user.password_hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}