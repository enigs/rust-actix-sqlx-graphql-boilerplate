use async_graphql::SimpleObject;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, SimpleObject)]
pub struct Token {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub access: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub refresh: String
}

impl Token {
    pub fn new() -> Self {
        Self::default()
    }
}