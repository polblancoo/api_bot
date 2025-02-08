use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub coingecko_api_key: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL debe estar configurado")?,
            coingecko_api_key: env::var("COINGECKO_API_KEY")
                .map_err(|_| "COINGECKO_API_KEY debe estar configurado")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| "SERVER_PORT debe ser un número válido")?,
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET debe estar configurado")?,
        })
    }
} 