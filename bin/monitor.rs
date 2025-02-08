use dotenv::dotenv;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use my_rust_api::{
    config::Config,
    db::init::{init_pool, init_database},
    create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Configurar logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciando servicio de monitoreo...");

    // Cargar configuración
    let config = Config::from_env()
        .expect("Error cargando la configuración");
    info!("Configuración cargada correctamente");

    // Inicializar base de datos
    let pool = init_pool(&config.database_url).await?;
    init_database(&pool).await?;

    // Crear router
    let app = create_router(pool).await;

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    // Create socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Servidor corriendo en http://{}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}