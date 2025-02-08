// 

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::jwt::Claims,
    db,
    models::{
        asset_pairs::CreateAssetPairRequest,
        price_alerts::CreatePriceAlertRequest,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub mod asset_pairs;
pub mod price_alerts;
pub mod users;
pub mod admin;

pub use self::{
    asset_pairs::{
        create_asset_pair,
        list_asset_pairs,
        get_asset_pair,
        update_asset_pair,
    },
    price_alerts::{
        create_price_alert,
        get_price_alerts,
        get_price_alert,
        update_price_alert,
        delete_price_alert,
    },
    users::{
        register,
        login,
    },
    admin::*,
};