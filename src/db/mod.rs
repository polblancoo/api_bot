use sqlx::{Pool, Postgres};
use tracing::{info, error};

pub mod init;
pub mod users;
pub mod asset_pairs;
pub mod api_credentials;
pub mod personal_data;
pub mod price_alerts;

pub async fn init_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    info!("Intentando conectar a la base de datos: {}", database_url);
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Ejecutar migraciones
    info!("Ejecutando migraciones desde: {}", std::env::current_dir()?.display());
    
    // Verificar que el archivo existe
    let migration_path = std::path::Path::new("migrations/20240320_initial_schema.sql");
    if migration_path.exists() {
        info!("Archivo de migración encontrado");
    } else {
        error!("¡Archivo de migración no encontrado!");
        return Err(sqlx::Error::Configuration(
            "Archivo de migración no encontrado".into()
        ));
    }

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => info!("Migraciones completadas exitosamente"),
        Err(e) => {
            error!("Error al ejecutar migraciones: {}", e);
            return Err(e.into());
        }
    }

    Ok(pool)
}
