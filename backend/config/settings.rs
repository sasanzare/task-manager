/** 
 * Contains all configuration structures for the application
 */
use serde::Deserialize;

/**
 * Main application configuration container
 * 
 * Combines all subsystem configurations including:
 * - Server settings (host/port)
 * - Database connection parameters
 * - CORS policy configuration
 */
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cors: CorsConfig,
}

/**
 * Server network configuration
 * 
 * Defines how the application binds to network interfaces
 */
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/**
 * Database connection configuration
 * 
 * Contains parameters for establishing database connections
 */
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

/**
 * CORS (Cross-Origin Resource Sharing) policy configuration
 * 
 * Defines which external origins can access the API
 */
#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origin: Vec<String>,
    pub allowed_methods: Vec<String>,
}