use anyhow::Result;
use chrono::Utc;
use nanoid::nanoid;
use sqlx::types::Json;
use std::sync::{Arc, RwLock};

use crate::Paseto;
use crate::DBManager;

impl Paseto {
    pub async fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = Paseto::select(manager).await {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        let settings = Self::new().upsert(manager).await?;
        Ok(Arc::new(RwLock::new(settings)))
    }

    pub async fn select(manager: &DBManager) -> Result<Self> {
        #[derive(Debug, sqlx::FromRow)]
        struct Settings {
            content: Json<Paseto>
        }

        let result = sqlx::query_as::<_, Settings>("SELECT content FROM settings WHERE module = 'PASETO'")
            .fetch_one(manager.reader())
            .await?
            .content
            .decrypt()?;

        Ok(result)
    }

    pub async fn upsert(&self, manager: &DBManager) -> Result<Self> {
        let id = nanoid!();
        let content = Json::from(self.encrypt()?);
        let timestamp = Utc::now();

        sqlx::query(r#"
            INSERT INTO settings (id, module, content, created_at, updated_at)
            VALUES ($1, 'PASETO', $2, $3, $4)
            ON CONFLICT (module)
            DO UPDATE SET content = $2, updated_at = $4
        "#).bind(id)
            .bind(content)
            .bind(timestamp)
            .bind(timestamp)
            .execute(manager.writer())
            .await?;

        Ok(self.clone())
    }
}