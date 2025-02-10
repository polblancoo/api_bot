use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::JsonValue, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EncryptedApiKey {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub key_hash: String,
    pub exchange: String,
    pub permissions: JsonValue,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub exchange: String,
    pub permissions: Vec<String>,
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

pub async fn create_api_key(
    pool: &PgPool,
    user_id: i32,
    req: CreateApiKeyRequest,
) -> Result<EncryptedApiKey, sqlx::Error> {
    let now = Utc::now();
    let permissions_json = serde_json::to_value(&req.permissions).unwrap();

    let encrypted_key = sqlx::query_as!(
        EncryptedApiKey,
        r#"
        INSERT INTO api_keys (
            user_id,
            name,
            key_hash,
            exchange,
            permissions,
            is_active,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, name, key_hash, exchange, permissions as "permissions: JsonValue", is_active, created_at, updated_at
        "#,
        user_id,
        req.name,
        req.api_key,
        req.exchange,
        permissions_json,
        true,
        now as _,
        now as _,
    )
    .fetch_one(pool)
    .await?;

    Ok(encrypted_key)
}

pub async fn get_api_key(
    pool: &PgPool,
    user_id: i32,
    api_key_id: i32,
) -> Result<Option<EncryptedApiKey>, sqlx::Error> {
    let encrypted_key = sqlx::query_as!(
        EncryptedApiKey,
        r#"
        SELECT id, user_id, name, key_hash, exchange, permissions as "permissions: JsonValue", is_active, created_at, updated_at
        FROM api_keys
        WHERE id = $1 AND user_id = $2
        "#,
        api_key_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(encrypted_key)
}

pub async fn list_api_keys(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<EncryptedApiKey>, sqlx::Error> {
    let encrypted_keys = sqlx::query_as!(
        EncryptedApiKey,
        r#"
        SELECT id, user_id, name, key_hash, exchange, permissions as "permissions: JsonValue", is_active, created_at, updated_at
        FROM api_keys
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(encrypted_keys)
}

pub async fn update_api_key(
    pool: &PgPool,
    user_id: i32,
    api_key_id: i32,
    req: &UpdateApiKeyRequest,
) -> Result<EncryptedApiKey, sqlx::Error> {
    let now = Utc::now();

    let mut updates = vec!["updated_at = $1"];
    let mut param_count = 2;

    let mut query = String::from(
        r#"
        UPDATE api_keys
        SET updated_at = $1
        "#
    );

    if let Some(name) = &req.name {
        query.push_str(&format!(", name = ${}", param_count));
        param_count += 1;
    }

    if let Some(api_key) = &req.api_key {
        query.push_str(&format!(", key_hash = ${}", param_count));
        param_count += 1;
    }

    if let Some(permissions) = &req.permissions {
        query.push_str(&format!(", permissions = ${}", param_count));
        param_count += 1;
    }

    if let Some(is_active) = req.is_active {
        query.push_str(&format!(", is_active = ${}", param_count));
        param_count += 1;
    }

    query.push_str(&format!(" WHERE id = ${} AND user_id = ${} RETURNING id, user_id, name, key_hash, exchange, permissions as \"permissions: JsonValue\", is_active, created_at, updated_at", param_count, param_count + 1));

    let mut query_builder = sqlx::query_as::<_, EncryptedApiKey>(&query)
        .bind(now);

    if let Some(name) = &req.name {
        query_builder = query_builder.bind(name);
    }

    if let Some(api_key) = &req.api_key {
        query_builder = query_builder.bind(api_key);
    }

    if let Some(permissions) = &req.permissions {
        let permissions_json = serde_json::to_value(permissions).unwrap();
        query_builder = query_builder.bind(permissions_json);
    }

    if let Some(is_active) = req.is_active {
        query_builder = query_builder.bind(is_active);
    }

    query_builder = query_builder.bind(api_key_id).bind(user_id);

    query_builder.fetch_one(pool).await
}

pub async fn delete_api_key(
    pool: &PgPool,
    api_key_id: i32,
    user_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM api_keys
        WHERE id = $1 AND user_id = $2
        "#,
        api_key_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
