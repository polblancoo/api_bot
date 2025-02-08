use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};

use super::Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub secret: String,
    pub enabled: bool,
    pub notification_types: Vec<String>,
}

pub struct WebhookManager {
    http_client: Client,
    pool: PgPool,
}

impl WebhookManager {
    pub fn new(pool: PgPool) -> Self {
        Self {
            http_client: Client::new(),
            pool,
        }
    }

    pub async fn send_webhook(
        &self,
        notification: &Notification,
        config: &WebhookConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "notification": notification,
            "timestamp": chrono::Utc::now(),
        });

        // Generar firma HMAC para seguridad
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(config.secret.as_bytes())?;
        mac.update(serde_json::to_string(&payload)?.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let response = self
            .http_client
            .post(&config.url)
            .header("X-Webhook-Signature", signature)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            error!(
                "Error enviando webhook a {}: {}",
                config.url,
                response.status()
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error HTTP: {}", response.status()),
            )));
        }

        info!("Webhook enviado exitosamente a {}", config.url);
        Ok(())
    }

    pub async fn get_webhook_configs(
        &self,
        user_id: i32,
    ) -> Result<Vec<WebhookConfig>, sqlx::Error> {
        sqlx::query_as!(
            WebhookConfig,
            r#"
            SELECT id, user_id, url, secret, enabled, notification_types
            FROM webhook_configs
            WHERE user_id = $1 AND enabled = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_webhook_config(
        &self,
        config: WebhookConfig,
    ) -> Result<WebhookConfig, sqlx::Error> {
        sqlx::query_as!(
            WebhookConfig,
            r#"
            INSERT INTO webhook_configs (user_id, url, secret, enabled, notification_types)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, url, secret, enabled, notification_types
            "#,
            config.user_id,
            config.url,
            config.secret,
            config.enabled,
            &config.notification_types as &[String],
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_webhook_config(
        &self,
        config: WebhookConfig,
    ) -> Result<WebhookConfig, sqlx::Error> {
        sqlx::query_as!(
            WebhookConfig,
            r#"
            UPDATE webhook_configs
            SET url = $1, secret = $2, enabled = $3, notification_types = $4
            WHERE id = $5 AND user_id = $6
            RETURNING id, user_id, url, secret, enabled, notification_types
            "#,
            config.url,
            config.secret,
            config.enabled,
            &config.notification_types as &[String],
            config.id,
            config.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_webhook_config(
        &self,
        id: i32,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM webhook_configs
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
