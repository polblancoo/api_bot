use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::models::personal_data::{PersonalData, CreatePersonalDataRequest};

pub async fn create_personal_data(
    pool: &PgPool,
    user_id: i32,
    req: &CreatePersonalDataRequest,
) -> Result<PersonalData, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as!(
        PersonalData,
        r#"
        INSERT INTO personal_data (user_id, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        user_id,
        req.email,
        now,
        now
    )
    .fetch_one(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<PersonalData, sqlx::Error> {
    sqlx::query_as!(
        PersonalData,
        r#"
        SELECT * FROM personal_data WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_by_user_id(pool: &PgPool, user_id: i32) -> Result<PersonalData, sqlx::Error> {
    sqlx::query_as!(
        PersonalData,
        r#"
        SELECT * FROM personal_data 
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_personal_data(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    email: String,
) -> Result<PersonalData, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as!(
        PersonalData,
        r#"
        UPDATE personal_data
        SET email = $1, updated_at = $2
        WHERE id = $3 AND user_id = $4
        RETURNING *
        "#,
        email,
        now,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}