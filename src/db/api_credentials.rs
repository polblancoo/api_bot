use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCredential {
    pub id: i32,
    pub user_id: i32,
    pub exchange_api_key: String,
    pub api_key_pasw: String,
    pub api_key_pasw2: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiCredentialRequest {
    pub exchange_api_key: String,
    pub api_key_pasw: String,
    pub api_key_pasw2: Option<String>,
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    cred: &CreateApiCredentialRequest,
) -> Result<ApiCredential, sqlx::Error> {
    let now = Utc::now();
    
    sqlx::query_as!(
        ApiCredential,
        r#"
        INSERT INTO api_credentials (
            user_id, exchange_api_key, api_key_pasw, api_key_pasw2, 
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $5)
        RETURNING *
        "#,
        user_id,
        cred.exchange_api_key,
        cred.api_key_pasw,
        cred.api_key_pasw2.as_ref(),
        now,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<ApiCredential, sqlx::Error> {
    sqlx::query_as!(
        ApiCredential,
        r#"
        SELECT * FROM api_credentials WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn list_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<ApiCredential>, sqlx::Error> {
    sqlx::query_as!(
        ApiCredential,
        r#"
        SELECT * FROM api_credentials 
        WHERE user_id = $1 
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    cred: &CreateApiCredentialRequest,
) -> Result<ApiCredential, sqlx::Error> {
    let now = Utc::now();
    
    sqlx::query_as!(
        ApiCredential,
        r#"
        UPDATE api_credentials
        SET exchange_api_key = $1, 
            api_key_pasw = $2, 
            api_key_pasw2 = $3,
            updated_at = $4
        WHERE id = $5 AND user_id = $6
        RETURNING *
        "#,
        cred.exchange_api_key,
        cred.api_key_pasw,
        cred.api_key_pasw2.as_ref(),
        now,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete(
    pool: &PgPool,
    id: i32,
    user_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM api_credentials
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ... implementar CRUD ... 