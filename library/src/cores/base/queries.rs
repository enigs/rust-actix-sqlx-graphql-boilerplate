use anyhow::Result;
use chrono::Utc;
use nanoid::nanoid;
use sqlx::types::Json;
use std::sync::{Arc, RwLock};

use crate::Base;
use crate::DBManager;

impl Base {
    pub async fn init(manager: &DBManager) -> Result<Arc<RwLock<Self>>> {
        if let Ok(settings) = Self::select(manager).await {
            return Ok(Arc::new(RwLock::new(settings)));
        }

        Ok(Arc::new(RwLock::new(Self::default())))
    }

    pub async fn select(manager: &DBManager) -> Result<Self> {
        #[derive(Debug, sqlx::FromRow)]
        struct Settings {
            content: Json<Base>
        }

        let result = sqlx::query_as::<_, Settings>("SELECT content FROM settings WHERE module = 'BASE'")
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
            VALUES ($1, 'BASE', $2, $3, $4)
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