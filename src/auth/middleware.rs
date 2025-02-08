use axum::{
    extract::FromRequestParts,
    http::Request,
    middleware::Next,
    response::Response,
};
use super::jwt::Claims;

pub async fn auth<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, (axum::http::StatusCode, String)> {
    let (mut parts, body) = req.into_parts();
    let claims = Claims::from_request_parts(&mut parts, &())
        .await
        .map_err(|e| e)?;
    
    let mut req = Request::from_parts(parts, body);
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}