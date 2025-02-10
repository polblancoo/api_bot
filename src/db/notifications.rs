use sqlx::PgPool;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use tracing::{debug, error};
use crate::models::notifications::{NotificationPreference, CreateWebhookRequest, WebhookConfig};

pub async fn get_notification_preferences(pool: &PgPool, user_id: i32) -> Result<NotificationPreference, sqlx::Error> {
    sqlx::query_as::<_, NotificationPreference>(
        r#"
        SELECT user_id, email_enabled, telegram_enabled, whatsapp_enabled,
               price_alerts_enabled, system_alerts_enabled, created_at, updated_at
        FROM notification_preferences
        WHERE user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn update_notification_preferences(
    pool: &PgPool,
    user_id: i32,
    email_enabled: Option<bool>,
    telegram_enabled: Option<bool>,
    whatsapp_enabled: Option<bool>,
    price_alerts_enabled: Option<bool>,
    system_alerts_enabled: Option<bool>,
) -> Result<NotificationPreference, sqlx::Error> {
    let mut updates = Vec::new();
    let mut params = vec![user_id];
    let mut param_count = 2;

    if let Some(email) = email_enabled {
        updates.push(format!("email_enabled = ${}", param_count));
        params.push(email as i32);
        param_count += 1;
    }

    if let Some(telegram) = telegram_enabled {
        updates.push(format!("telegram_enabled = ${}", param_count));
        params.push(telegram as i32);
        param_count += 1;
    }

    if let Some(whatsapp) = whatsapp_enabled {
        updates.push(format!("whatsapp_enabled = ${}", param_count));
        params.push(whatsapp as i32);
        param_count += 1;
    }

    if let Some(price) = price_alerts_enabled {
        updates.push(format!("price_alerts_enabled = ${}", param_count));
        params.push(price as i32);
        param_count += 1;
    }

    if let Some(system) = system_alerts_enabled {
        updates.push(format!("system_alerts_enabled = ${}", param_count));
        params.push(system as i32);
        param_count += 1;
    }

    updates.push(format!("updated_at = now()"));

    let query = format!(
        r#"
        UPDATE notification_preferences
        SET {}
        WHERE user_id = $1
        RETURNING user_id, email_enabled, telegram_enabled, whatsapp_enabled,
                 price_alerts_enabled, system_alerts_enabled, created_at, updated_at
        "#,
        updates.join(", ")
    );

    sqlx::query_as::<_, NotificationPreference>(&query)
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn create_webhook(
    pool: &PgPool,
    user_id: i32,
    request: &CreateWebhookRequest,
) -> Result<WebhookConfig, sqlx::Error> {
    debug!("Creando webhook para usuario {}", user_id);
    debug!("Request: {:?}", request);
    
    let webhook_id = Uuid::new_v4();
    debug!("Webhook ID generado: {}", webhook_id);

    let notification_types = match serde_json::to_value(&request.notification_types) {
        Ok(val) => {
            debug!("Notification types serializados: {:?}", val);
            val
        },
        Err(e) => {
            error!("Error al serializar notification_types: {}", e);
            return Err(sqlx::Error::Protocol(format!("Error al serializar notification_types: {}", e)));
        }
    };

    debug!("Ejecutando query de inserción");
    let result = sqlx::query_as::<_, WebhookConfig>(
        r#"
        INSERT INTO webhooks (
            id, user_id, url, secret, notification_types, enabled,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, true, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, user_id, url, secret, notification_types, enabled, created_at, updated_at
        "#
    )
    .bind(webhook_id)
    .bind(user_id)
    .bind(&request.url)
    .bind(&request.secret)
    .bind(notification_types)
    .fetch_one(pool)
    .await;

    match &result {
        Ok(webhook) => debug!("Webhook creado exitosamente: {:?}", webhook),
        Err(e) => error!("Error al crear webhook en la base de datos: {}", e),
    }

    result
}

pub async fn list_webhooks(pool: &PgPool, user_id: i32) -> Result<Vec<WebhookConfig>, sqlx::Error> {
    sqlx::query_as::<_, WebhookConfig>(
        r#"
        SELECT id, user_id, url, secret, notification_types as "notification_types",
               enabled, created_at, updated_at
        FROM webhooks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn delete_webhook(pool: &PgPool, user_id: i32, webhook_id: Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM webhooks
        WHERE id = $1 AND user_id = $2
        "#
    )
    .bind(webhook_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(sqlx::Error::RowNotFound)
    } else {
        Ok(())
    }
}
