use sqlx::{Pool, Postgres};
use tracing::{info, error};

pub mod init;
pub mod users;
pub mod personal_data;
pub mod api_keys;
pub mod notifications;

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
        info!("Archivo de migración encontrado: {}", migration_path.display());
        
        // Leer el contenido del archivo
        let sql = std::fs::read_to_string(migration_path)?;
        
        // Ejecutar el SQL
        sqlx::query(&sql).execute(&pool).await?;
        info!("Migraciones ejecutadas exitosamente");
    } else {
        error!("No se encontró el archivo de migración: {}", migration_path.display());
    }

    Ok(pool)
}
