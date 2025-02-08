use redis::{aio::Connection, Client, RedisError};
use serde_json::json;
use tracing::{error, info};

use super::{Notification, NotificationType};

pub struct NotificationQueue {
    redis_client: Client,
}

impl NotificationQueue {
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let redis_client = Client::open(redis_url)?;
        Ok(Self { redis_client })
    }

    pub async fn get_connection(&self) -> Result<Connection, RedisError> {
        self.redis_client.get_async_connection().await
    }

    pub async fn push_notification(&self, notification: &Notification) -> Result<(), RedisError> {
        let mut conn = self.get_connection().await?;
        
        let notification_json = serde_json::to_string(notification).map_err(|e| {
            error!("Error serializando notificación: {}", e);
            RedisError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error serializando notificación",
            ))
        })?;

        let queue_key = match notification.notification_type {
            NotificationType::PriceAlert => "queue:price_alerts",
            NotificationType::MarketSentiment => "queue:market_sentiment",
            NotificationType::StrategyUpdate => "queue:strategy_updates",
            NotificationType::TradeExecution => "queue:trade_execution",
            NotificationType::SystemAlert => "queue:system_alerts",
        };

        redis::cmd("LPUSH")
            .arg(queue_key)
            .arg(notification_json)
            .query_async(&mut conn)
            .await?;

        info!(
            "Notificación agregada a la cola {}: {:?}",
            queue_key, notification
        );

        Ok(())
    }

    pub async fn pop_notification(
        &self,
        notification_type: NotificationType,
    ) -> Result<Option<Notification>, RedisError> {
        let mut conn = self.get_connection().await?;

        let queue_key = match notification_type {
            NotificationType::PriceAlert => "queue:price_alerts",
            NotificationType::MarketSentiment => "queue:market_sentiment",
            NotificationType::StrategyUpdate => "queue:strategy_updates",
            NotificationType::TradeExecution => "queue:trade_execution",
            NotificationType::SystemAlert => "queue:system_alerts",
        };

        let notification_json: Option<String> = redis::cmd("RPOP")
            .arg(queue_key)
            .query_async(&mut conn)
            .await?;

        match notification_json {
            Some(json_str) => {
                let notification: Notification = serde_json::from_str(&json_str).map_err(|e| {
                    error!("Error deserializando notificación: {}", e);
                    RedisError::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error deserializando notificación",
                    ))
                })?;
                Ok(Some(notification))
            }
            None => Ok(None),
        }
    }

    pub async fn get_queue_length(
        &self,
        notification_type: NotificationType,
    ) -> Result<i64, RedisError> {
        let mut conn = self.get_connection().await?;

        let queue_key = match notification_type {
            NotificationType::PriceAlert => "queue:price_alerts",
            NotificationType::MarketSentiment => "queue:market_sentiment",
            NotificationType::StrategyUpdate => "queue:strategy_updates",
            NotificationType::TradeExecution => "queue:trade_execution",
            NotificationType::SystemAlert => "queue:system_alerts",
        };

        redis::cmd("LLEN")
            .arg(queue_key)
            .query_async(&mut conn)
            .await
    }
}
