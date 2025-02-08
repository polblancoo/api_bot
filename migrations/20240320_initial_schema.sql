-- Eliminar tablas existentes si las hay
DROP TABLE IF EXISTS strategies;
DROP TABLE IF EXISTS asset_pairs;
DROP TABLE IF EXISTS price_alerts;
DROP TABLE IF EXISTS personal_data;
DROP TABLE IF EXISTS api_credentials;
DROP TABLE IF EXISTS users;

-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    telegram_id VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL,
    last_login TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- API credentials table
CREATE TABLE api_credentials (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    exchange_api_key TEXT NOT NULL,
    api_key_pasw TEXT NOT NULL,
    api_key_pasw2 TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Personal data table
CREATE TABLE personal_data (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id),
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Asset pairs table
CREATE TABLE IF NOT EXISTS asset_pairs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    base_asset VARCHAR(10) NOT NULL,
    quote_asset VARCHAR(10) NOT NULL,
    slip_percentage DECIMAL NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(user_id, base_asset, quote_asset)
);

-- Price alerts table
CREATE TABLE price_alerts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    asset VARCHAR(20) NOT NULL,
    target_price DECIMAL NOT NULL,
    condition VARCHAR(10) NOT NULL CHECK (condition IN ('above', 'below')),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    trigger_price DECIMAL
);

-- Índices
CREATE INDEX idx_price_alerts_user ON price_alerts(user_id);
CREATE INDEX idx_asset_pairs_user ON asset_pairs(user_id); 