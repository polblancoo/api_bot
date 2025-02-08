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
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
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

    // Eliminar la restricción única si existe
    sqlx::query!(
        r#"
        ALTER TABLE asset_pairs
        DROP CONSTRAINT IF EXISTS asset_pairs_user_id_base_asset_quote_asset_key
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