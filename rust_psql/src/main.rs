mod db;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    // Get the database URL from the environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create a connection pool to the PostgreSQL database
    let _pool = db::create_pool(&database_url).await?;

    Ok(())
}
