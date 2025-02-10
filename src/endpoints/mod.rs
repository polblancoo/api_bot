use axum::Router;

use crate::app_state::AppState;

pub mod api_keys;
pub mod auth;
pub mod users;
pub mod notifications;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(api_keys::api_keys_router())
        .merge(auth::auth_router())
        .merge(users::users_router())
        .with_state(state)
}