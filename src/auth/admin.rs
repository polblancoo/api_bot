use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};

use crate::{auth::jwt::Claims, endpoints::AppState};

pub async fn require_admin<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Claims not found in request".to_string(),
        ))?;

    let user = sqlx::query!(
        r#"
        SELECT is_admin
        FROM users
        WHERE id = $1
        "#,
        claims.user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error fetching user".to_string(),
        )
    })?;

    if !user.is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            "Admin privileges required".to_string(),
        ));
    }

    Ok(next.run(req).await)
}
