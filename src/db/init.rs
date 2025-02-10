use sqlx::{Pool, Postgres, Row, PgPool};
use tracing::info;

pub async fn init_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    info!("Inicializando pool de base de datos...");
    
    // Conectar a la base de datos
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("Conexión establecida");

    // Verificar que las tablas existen
    let tables = sqlx::query(
        r#"
        SELECT tablename FROM pg_catalog.pg_tables 
        WHERE schemaname = 'public'
        ORDER BY tablename;
        "#
    )
    .fetch_all(&pool)
    .await?;

    info!("Tablas encontradas:");
    for table in tables {
        let name: String = table.get("tablename");
        info!("  - {}", name);
    }

    Ok(pool)
}

pub async fn init_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            is_admin BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create personal_data table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS personal_data (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            email VARCHAR(255) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create api_keys table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS api_keys (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            name VARCHAR(255) NOT NULL,
            key_hash VARCHAR(255) NOT NULL,
            exchange VARCHAR(50) NOT NULL,
            permissions JSONB NOT NULL,
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id, name)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create user_encryption_keys table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS user_encryption_keys (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            key_hash BYTEA NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create notification_preferences table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS notification_preferences (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            email_enabled BOOLEAN DEFAULT true,
            telegram_enabled BOOLEAN DEFAULT true,
            whatsapp_enabled BOOLEAN DEFAULT false,
            price_alerts_enabled BOOLEAN DEFAULT true,
            system_alerts_enabled BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create webhooks table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS webhooks (
            id UUID PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            url TEXT NOT NULL,
            secret TEXT NOT NULL,
            notification_types JSONB NOT NULL,
            enabled BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create asset_pairs table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS asset_pairs (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            base_asset VARCHAR(50) NOT NULL,
            quote_asset VARCHAR(50) NOT NULL,
            slip_percentage DECIMAL NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create price_alerts table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS price_alerts (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            asset_pair_id INTEGER NOT NULL REFERENCES asset_pairs(id),
            target_price DECIMAL NOT NULL,
            alert_type VARCHAR(50) NOT NULL,
            is_active BOOLEAN DEFAULT true,
            trigger_price DECIMAL,
            triggered_at TIMESTAMP WITH TIME ZONE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}