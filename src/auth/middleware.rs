use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
};
use tracing::debug;

use crate::auth::jwt::{self, Claims};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("Headers: {:?}", parts.headers);
        
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| {
                debug!("Missing Authorization header");
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing Authorization header".to_string(),
                )
            })?;

        let auth_str = auth_header
            .to_str()
            .map_err(|e| {
                debug!("Invalid Authorization header: {}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        // Verificar que el header comienza con "Bearer "
        if !auth_str.starts_with("Bearer ") {
            debug!("Authorization header must start with Bearer");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Authorization header must start with Bearer".to_string(),
            ));
        }

        let token = &auth_str[7..]; // Saltar "Bearer "
        debug!("Token: {}", token);

        jwt::verify_token(token)
            .map_err(|e| {
                debug!("Token verification failed: {:?}", e);
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            })
    }
}