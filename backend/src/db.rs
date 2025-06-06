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