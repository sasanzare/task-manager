/**
 * Main application entry point module
 * 
 * Configures and launches the Actix-Web HTTP server with:
 * - Environment-based configuration
 * - Database connection pooling
 * - CORS policy management
 * - Route handlers
 */
use actix_cors::Cors;
use actix_web::{get, http::Method, http::header, web, App, HttpResponse, HttpServer, Responder};
use std::str::FromStr;

// Application modules
mod db;
#[path = "../config/mod.rs"] mod config;

/**
 * Root endpoint handler
 * 
 * @returns HTTP 200 response with JSON greeting
 */
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello "
    }))
}

/**
 * Application main entry point
 * 
 * @returns IO Result indicating success or failure
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment-specific configuration
    let config = config::load_config().expect("Failed to load configuration");
    println!("🟢 Running in {} mode", std::env::var("APP_ENV").unwrap_or("dev".into()));

    // Initialize database connection pool
    let pool = db::establish_connection(&config.database.url)
        .await
        .expect("Failed to connect to database");
    println!("✅ Database connection established successfully");

    // Configure and start HTTP server
    HttpServer::new(move || {
        // Configure CORS policy from settings
        let mut cors = Cors::default();
        
        // Handle wildcard origin (*)
        if config.cors.allowed_origin.contains(&"*".to_string()) {
            cors = cors.allow_any_origin();
        } else {
            // Add each allowed origin individually
            for origin in &config.cors.allowed_origin {
                cors = cors.allowed_origin(origin);
            }
        }

        // Configure allowed methods and headers
        cors = cors
            .allowed_methods(
                config.cors.allowed_methods
                    .iter()
                    .map(|m| Method::from_str(m).unwrap())
                    .collect::<Vec<Method>>()
            )
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600); // 1 hour cache
        
        // Build application instance
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share database pool
            .wrap(cors) // Apply CORS middleware
            .service(hello) // Register route handler
    })
    .bind((config.server.host, config.server.port))? // Bind to configured host:port
    .run() // Start server
    .await
}