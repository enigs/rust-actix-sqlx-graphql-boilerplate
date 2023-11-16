pub mod form;
pub mod queries;

use arraygen::Arraygen;
use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

pub use form::{BaseForm, BaseError};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[derive(macros::SetCipher, macros::SetIsEmpty, macros::SetMutate, sqlx::Type)]
#[derive(SimpleObject)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "JSONB")]
pub struct Base {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub api_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub web_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub admin_url: String
}

impl Base {
    pub fn get_api_url(&self) -> Cow<str> {
        Cow::from(&self.api_url)
    }

    pub fn get_web_url(&self) -> Cow<str> {
        Cow::from(&self.web_url)
    }

    pub fn get_admin_url(&self) -> Cow<str> {
        Cow::from(&self.admin_url)
    }
}