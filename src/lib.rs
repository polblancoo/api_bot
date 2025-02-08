use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use endpoints::{
    AppState,
    users::{register, login},
    asset_pairs::{
        create_asset_pair,
        list_asset_pairs,
        get_asset_pair,
        update_asset_pair,
        delete_asset_pair,
    },
    price_alerts::{
        create_price_alert,
        get_price_alerts,
        get_price_alert,
        update_price_alert,
        delete_price_alert,
    },
    admin::{
        get_users,
        get_all_asset_pairs,
        get_all_alerts,
    },
};
use tower_http::cors::{Any, CorsLayer};
use crate::auth::{middleware::auth, admin::require_admin};

pub mod auth;
pub mod config;
pub mod db;
pub mod endpoints;
pub mod models;

pub async fn create_router(pool: sqlx::PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app_state = AppState { pool };

    // Rutas públicas
    let public_routes = Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login));

    // Rutas protegidas
    let protected_routes = Router::new()
        .route(
            "/asset-pairs",
            post(create_asset_pair).get(list_asset_pairs),
        )
        .route(
            "/asset-pairs/:id",
            get(get_asset_pair)
                .put(update_asset_pair)
                .delete(delete_asset_pair),
        )
        .route(
            "/price-alerts",
            post(create_price_alert).get(get_price_alerts),
        )
        .route(
            "/price-alerts/:id",
            get(get_price_alert)
                .put(update_price_alert)
                .delete(delete_price_alert),
        )
        .layer(middleware::from_fn(auth));

    // Rutas de administrador
    let admin_routes = Router::new()
        .route("/admin/users", get(get_users))
        .route("/admin/asset-pairs", get(get_all_asset_pairs))
        .route("/admin/price-alerts", get(get_all_alerts))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_admin,
        ))
        .layer(middleware::from_fn(auth));

    // Combinar todas las rutas
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(cors)
        .with_state(app_state)
}