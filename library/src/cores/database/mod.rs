use anyhow::Result;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[derive(Clone, Debug)]
pub struct DBManager {
    pub reader: Pool<Postgres>,
    pub writer: Pool<Postgres>
}

impl DBManager {
    pub async fn init() -> Result<Self> {
        // Set writer url
        let writer = std::env::var("DATABASE_WRITE_URL")?;

        // Set reader url
        let reader  = std::env::var("DATABASE_READ_URL")
            .unwrap_or(writer.clone());

        // Check if any of the database connection is empty
        if writer.is_empty() || reader.is_empty() {
            return Err(anyhow::anyhow!("Invalid database url configuration"));
        }

        // Set writer pool
        let writer = PgPoolOptions::new()
            .connect(&writer)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create writer pool: {}", e))?;

        // Set reader pool
        let reader = PgPoolOptions::new()
            .connect(&reader)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create reader pool: {}", e))?;

        // Return database manager
        Ok(Self { reader, writer })
    }

    pub fn reader(&self) -> &Pool<Postgres> {
        &self.reader
    }

    pub fn writer(&self) -> &Pool<Postgres> {
        &self.writer
    }
}