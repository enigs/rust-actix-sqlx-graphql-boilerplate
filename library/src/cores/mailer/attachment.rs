use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[derive(sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "JSONB")]
pub struct MailerAttachment {
    pub filename: String,
    pub name: String
}