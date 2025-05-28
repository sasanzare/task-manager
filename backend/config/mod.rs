/** 
 * Application configuration module 
 * Handles loading and managing application settings
 */
mod env;
mod settings;

use config::Config;
use std::path::Path;

/** Re-export AppConfig for public use */
pub use settings::AppConfig;

/**
 * Loads application configuration based on current environment
 * 
 * Reads environment-specific config file from config/{env}.toml
 * 
 * @returns Result with AppConfig on success or ConfigError on failure
 * @error ConfigError::NotFound if config file doesn't exist
 * @error ConfigError::Deserialization if config parsing fails
 */
pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    // Load current environment (dev/prod)
    let env = env::Environment::load();
    
    // Build config file path based on environment
    let config_path = format!("config/{}.toml", env.as_str());

    // Verify config file exists
    if !Path::new(&config_path).exists() {
        return Err(config::ConfigError::NotFound(config_path));
    }

    // Build and parse config
    let cfg = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    // Deserialize config into AppConfig struct
    cfg.try_deserialize()
}