mod db;
use crate::db::init_db;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load env vars from .env if available
    dotenv::dotenv().ok();

    // Initlialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    Ok(())
}

