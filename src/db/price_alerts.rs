use sqlx::PgPool;
use crate::models::price_alerts::{PriceAlert, CreatePriceAlertRequest};

pub async fn create_price_alert(
    pool: &PgPool,
    user_id: i32,
    req: &CreatePriceAlertRequest,
) -> Result<PriceAlert, sqlx::Error> {
    sqlx::query_as!(
        PriceAlert,
        r#"
        INSERT INTO price_alerts (
            user_id,
            asset,
            target_price,
            condition,
            trigger_price,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING *
        "#,
        user_id,
        req.asset,
        req.target_price,
        req.condition,
        req.trigger_price
    )
    .fetch_one(pool)
    .await
}

pub async fn list_price_alerts(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<PriceAlert>, sqlx::Error> {
    sqlx::query_as!(
        PriceAlert,
        r#"
        SELECT *
        FROM price_alerts
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_price_alert(
    pool: &PgPool,
    id: i32,
) -> Result<PriceAlert, sqlx::Error> {
    sqlx::query_as!(
        PriceAlert,
        r#"
        SELECT *
        FROM price_alerts
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_price_alert(
    pool: &PgPool,
    id: i32,
    user_id: i32,
    req: &CreatePriceAlertRequest,
) -> Result<PriceAlert, sqlx::Error> {
    sqlx::query_as!(
        PriceAlert,
        r#"
        UPDATE price_alerts
        SET asset = $1,
            target_price = $2,
            condition = $3,
            trigger_price = $4,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $5 AND user_id = $6
        RETURNING *
        "#,
        req.asset,
        req.target_price,
        req.condition,
        req.trigger_price,
        id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_price_alert(
    pool: &PgPool,
    id: i32,
    user_id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM price_alerts
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_all_price_alerts_admin(pool: &PgPool) -> Result<Vec<PriceAlert>, sqlx::Error> {
    sqlx::query_as!(
        PriceAlert,
        r#"
        SELECT id, user_id, asset, target_price as "target_price: _", condition, trigger_price as "trigger_price: _", created_at, updated_at
        FROM price_alerts
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await
}