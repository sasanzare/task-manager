// /**
//  * Database module
//  * 
//  * Handles all database connection and pooling functionality
//  * for PostgreSQL database.
//  */
// use sqlx::postgres::PgPoolOptions;
// use sqlx::{Pool, Postgres};

// /**
//  * Establishes a connection pool to PostgreSQL database
//  * 
//  * Creates a pool of database connections with configured maximum connections.
//  * 
//  * @param database_url - PostgreSQL connection string in format:
//  *                      "postgres://user:password@host:port/database"
//  * @returns Result containing connection pool or sqlx::Error on failure
//  */
// pub async fn establish_connection(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
//     // Create connection pool with max 5 connections
//     PgPoolOptions::new()
//         .max_connections(5) // Maximum number of connections in pool
//         .connect(database_url) // Connect using provided URL
//         .await // Wait for connection establishment
// }



use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, migrate::Migrator};
use std::path::Path;

pub async fn establish_connection(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Path to migrations folder
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    
    // Run all pending migrations
    migrator.run(pool).await?;
    
    Ok(())
}