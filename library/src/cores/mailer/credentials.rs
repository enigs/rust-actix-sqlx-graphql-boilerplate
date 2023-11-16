use arraygen::Arraygen;
use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[derive(macros::SetCipher, macros::SetIsEmpty, macros::SetMutate, sqlx::Type)]
#[derive(SimpleObject)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "JSONB")]
pub struct MailerCredentials {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub smtp_host: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub service: String,
}

