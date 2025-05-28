/** Environment configuration module */
use std::env;

/** Application runtime environment */
#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Dev,    // Development environment 
    Prod,  // Production environment
}

impl Environment {
    /** Returns environment as string ("dev" or "prod") */
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
        }
    }

    /** 
     * Loads environment from APP_ENV variable 
     * Defaults to "dev" if not set
     */
    pub fn load() -> Self {
        dotenv::dotenv().ok();
        match env::var("APP_ENV").unwrap_or_else(|_| "dev".into()).as_str() {
            "prod" => Environment::Prod,
            _ => Environment::Dev,
        }
    }
}