use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use super::Notification;

type Users = Arc<RwLock<HashMap<i32, mpsc::Sender<Message>>>>;

#[derive(Debug, Serialize, Deserialize)]
struct WebSocketMessage {
    message_type: String,
    payload: serde_json::Value,
}

pub struct WebSocketServer {
    users: Users,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn handle_socket(
        &self,
        socket: WebSocket,
        user_id: i32,
    ) {
        let (mut sender, mut receiver) = socket.split();
        let (tx, mut rx) = mpsc::channel::<Message>(100);

        // Almacenar el sender para este usuario
        self.users.write().await.insert(user_id, tx);

        // Task para enviar mensajes al WebSocket
        let send_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = sender.send(message).await {
                    error!("Error enviando mensaje por WebSocket: {}", e);
                    break;
                }
            }
        });

        // Task para recibir mensajes del WebSocket
        let users = self.users.clone();
        let receive_task = tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
                        if let Ok(ws_message) = serde_json::from_str::<WebSocketMessage>(&text) {
                            info!("Mensaje recibido de usuario {}: {:?}", user_id, ws_message);
                            // Aquí puedes manejar diferentes tipos de mensajes
                        }
                    }
                    Message::Close(_) => {
                        users.write().await.remove(&user_id);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Esperar a que cualquiera de las tasks termine
        tokio::select! {
            _ = send_task => {},
            _ = receive_task => {},
        }

        // Limpiar la conexión
        self.users.write().await.remove(&user_id);
    }

    pub async fn broadcast_notification(&self, notification: &Notification) {
        let message = WebSocketMessage {
            message_type: "notification".to_string(),
            payload: serde_json::to_value(notification).unwrap_or_default(),
        };

        if let Ok(message_json) = serde_json::to_string(&message) {
            let ws_message = Message::Text(message_json);
            let users = self.users.read().await;
            
            if let Some(sender) = users.get(&notification.user_id) {
                if let Err(e) = sender.send(ws_message).await {
                    error!(
                        "Error enviando notificación al usuario {}: {}",
                        notification.user_id, e
                    );
                }
            }
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_id: i32,
    ws_server: Arc<WebSocketServer>,
) -> Response {
    ws.on_upgrade(move |socket| async move {
        ws_server.handle_socket(socket, user_id).await;
    })
}
