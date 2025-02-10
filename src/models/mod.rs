pub mod users;
pub mod notifications;
pub mod asset_pairs;
pub mod price_alerts;
pub mod personal_data;
pub mod api_keys;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            message: "Operation successful".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: "error".to_string(),
            message,
            data: None,
        }
    }
}