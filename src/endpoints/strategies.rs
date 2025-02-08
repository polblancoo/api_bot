use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use crate::{
    models::user::Claims,
    db::strategies::{Strategy, CreateStrategyRequest},
};

// ... implementar CRUD endpoints ... 