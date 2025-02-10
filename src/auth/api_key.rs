use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    TypedHeader,
};
use std::env;
use tracing::debug;
use lazy_static::lazy_static;

lazy_static! {
    static ref API_KEY: String = env::var("API_KEY").expect("API_KEY must be set");
}

pub struct ApiKey(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Obtener el header de autorización
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| {
                debug!("API key no proporcionada");
                (StatusCode::UNAUTHORIZED, "API key no proporcionada".to_string())
            })?;

        // Verificar que la API key sea correcta
        if bearer.token() == API_KEY.as_str() {
            debug!("API key válida");
            Ok(ApiKey(bearer.token().to_string()))
        } else {
            debug!("API key inválida");
            Err((StatusCode::UNAUTHORIZED, "API key inválida".to_string()))
        }
    }
}
