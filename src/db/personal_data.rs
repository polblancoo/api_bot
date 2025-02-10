use crate::models::personal_data::UpdatePersonalDataRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalData {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub async fn create_personal_data(
    pool: &PgPool,
    user_id: i32,
    email: &str,
) -> Result<PersonalData, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as!(
        PersonalData,
        r#"
        INSERT INTO personal_data (user_id, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, email, created_at, updated_at
        "#,
        user_id,
        email,
        now as _,
        now as _,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_personal_data(
    pool: &PgPool,
    user_id: i32,
) -> Result<Option<PersonalData>, sqlx::Error> {
    sqlx::query_as!(
        PersonalData,
        r#"
        SELECT * FROM personal_data 
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_personal_data(
    pool: &PgPool,
    user_id: i32,
    req: &UpdatePersonalDataRequest,
) -> Result<PersonalData, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as!(
        PersonalData,
        r#"
        UPDATE personal_data
        SET email = $1, updated_at = $2
        WHERE user_id = $3
        RETURNING id, user_id, email, created_at, updated_at
        "#,
        req.email,
        now as _,
        user_id
    )
    .fetch_one(pool)
    .await
}