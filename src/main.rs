use std::net::SocketAddr;
use dotenv::dotenv;
use my_rust_api::{create_router, db::init::{init_pool, init_database}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initialize database pool
    let pool = init_pool(&database_url).await.expect("Failed to create pool");

    // Initialize database schema
    init_database(&pool).await.expect("Failed to initialize database");

    // Create router
    let app = create_router(pool).await;

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    // Create socket address
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Server running on http://{}", addr);

    // Start server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
