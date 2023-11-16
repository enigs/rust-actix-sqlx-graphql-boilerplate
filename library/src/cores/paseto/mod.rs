pub mod form;
pub mod queries;

use anyhow::Result;
use arraygen::Arraygen;
use async_graphql::SimpleObject;
use chrono::{DateTime, Duration, Utc};
use pasetolib::tokens::{validate_local_token, PasetoBuilder, TimeBackend};
use serde::{Serialize, Deserialize};

use crate::prelude::{CustomRole, CustomStatus};
use crate::{Errors, Response};
use crate::Claims;
use crate::Token;

pub use form::{PasetoForm, PasetoError};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[derive(macros::SetCipher, macros::SetIsEmpty, macros::SetMutate, sqlx::Type)]
#[derive(SimpleObject)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "JSONB")]
pub struct Paseto {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub app_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub access_token_key_unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub access_token_key_time: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub access_token_key_signing: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_unit: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_time: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_signing: String
}

impl Paseto {
    pub fn new() -> Self {
        Self {
            app_name: config::app_name(),
            access_token_key_unit: config::PASETO_ACCESS_TOKEN_KEY_UNIT.to_string(),
            access_token_key_time: config::PASETO_ACCESS_TOKEN_KEY_TIME.to_string(),
            access_token_key_signing: config::PASETO_ACCESS_TOKEN_KEY_SIGNING.to_string(),
            refresh_token_key_unit: config::PASETO_REFRESH_TOKEN_KEY_UNIT.to_string(),
            refresh_token_key_time: config::PASETO_REFRESH_TOKEN_KEY_TIME.to_string(),
            refresh_token_key_signing: config::PASETO_REFRESH_TOKEN_KEY_SIGNING.to_string(),
        }
    }

    pub fn get_duration<U, T>(unit: U, time: T, min: i64) -> Duration
        where U: ToString,
              T: ToString
    {
        // Set unit
        let unit = unit
            .to_string()
            .parse::<i64>()
            .unwrap_or_else(|_| 0.max(min));

        // Set time
        let time = time.to_string();

        match time.to_lowercase().as_str() {
            "minutes" | "minute" => Duration::minutes(unit),
            "hours" | "hour" => Duration::hours(unit),
            "days" | "day" => Duration::days(unit),
            "weeks" | "week" => Duration::weeks(unit),
            "months" | "month" => Duration::days(unit * 30),
            _ => Duration::minutes(unit)
        }
    }

    pub fn get_expiration_date(duration: &Duration) -> DateTime<Utc> {
        Utc::now().checked_add_signed(*duration).unwrap_or_default()
    }

    pub fn get_access_token_expiry(&self) -> DateTime<Utc> {
        let duration = Self::get_duration(&self.access_token_key_unit, &self.access_token_key_time, 5);
        Self::get_expiration_date(&duration)
    }

    pub fn get_refresh_token_expiry(&self) -> DateTime<Utc> {
        let duration = Self::get_duration(&self.refresh_token_key_unit, &self.refresh_token_key_time, 30);
        Self::get_expiration_date(&duration)
    }

    pub fn generate_tokens<I, C>(&self, aid:I, claims: &C) -> async_graphql::Result<Token>
        where I: ToString,
              C: serde::Serialize + Clone
    {
        // Convert claims to value
        let claims = serde_json::to_value(&(*claims).clone()).unwrap();

        // Retrieve aid (actor id)
        let aid = aid.to_string();

        // Set app name
        let app_name = self.app_name.clone();

        // Retrieve access token values
        let response = Response::InternalServerError;
        let error = "Unable to generate access token";
        let access_token_duration = Self::get_duration(&self.access_token_key_unit, &self.access_token_key_time, 5);
        let access_token_expiry = Self::get_expiration_date(&access_token_duration);
        let access_token_signing = match base64_url::decode(&self.access_token_key_signing.clone()) {
            Ok(d) => d,
            Err(_) => return Err(Errors::to(response, error))
        };

        // Set access token
        let response = Response::InternalServerError;
        let error = "Unable to generate access token";
        let access_token = match PasetoBuilder::new()
            .set_encryption_key(&access_token_signing[..])
            .set_expiration(&access_token_expiry)
            .set_subject(&aid)
            .set_footer(&format!("key-id:{app_name}"))
            .set_claim("data", claims.clone())
            .build() {
            Ok(d) => d,
            Err(_) => return Err(Errors::to(response, error))
        };

        // Retrieve refresh token values
        let response = Response::InternalServerError;
        let error = "Unable to generate refresh token";
        let refresh_token_duration = Self::get_duration(&self.refresh_token_key_unit, &self.refresh_token_key_time, 30);
        let refresh_token_expiry = Self::get_expiration_date(&refresh_token_duration);
        let refresh_token_signing = match base64_url::decode(&self.refresh_token_key_signing.clone()) {
            Ok(d) => d,
            Err(_) => return Err(Errors::to(response, error))
        };

        // Set refresh token
        let response = Response::InternalServerError;
        let error = "Unable to generate refresh token";
        let refresh_token = match PasetoBuilder::new()
            .set_encryption_key(&refresh_token_signing[..])
            .set_expiration(&refresh_token_expiry)
            .set_subject(&aid)
            .set_footer(&format!("key-id:{app_name}"))
            .set_claim("data", claims)
            .build() {
            Ok(d) => d,
            Err(_) => return Err(Errors::to(response, error))
        };

        // Create mutable token
        let tokens = Token {
            access: access_token,
            refresh: refresh_token
        };

        // Return tokens
        Ok(tokens)
    }

    pub fn validate_access_token<R, S>(&self, token: &str) -> Result<Claims<R, S>>
        where R: CustomRole,
              S: CustomStatus
    {
        // Decrypt access token signing
        let access_token_signing = match base64_url::decode(&self.access_token_key_signing) {
            Ok(value) => value,
            Err(error) => return Err(anyhow::anyhow!(error))
        };

        // Set default values
        let app_name = format!("key-id:{}", self.app_name);
        let footer = Some(app_name.as_str());
        let key = &access_token_signing[..];
        let backend = TimeBackend::Chrono;

        // Verify token
        let result = match validate_local_token(token, footer, key, &backend) {
            Ok(value) => value,
            Err(error) => return match error
                .to_string()
                .to_lowercase()
                .as_str() == "this token is expired (exp claim)."
            {
                true => Err(anyhow::anyhow!("Your authentication token has expired")),
                false => Err(anyhow::anyhow!("Invalid authentication token"))
            }
        };

        // Retrieve values from paseto
        let result = match result.get("data") {
            Some(value) => value.to_owned(),
            None => return Err(anyhow::anyhow!("Invalid authentication token"))
        };

        // Return value to custom struct
        let claims = Claims::from(result);

        // Return claims
        Ok(claims)
    }

    pub fn validate_refresh_token<R, S>(&self, token: &str) -> Result<Claims<R, S>>
        where R: CustomRole,
              S: CustomStatus
    {
        // Decrypt refresh token signing
        let refresh_token_signing = match base64_url::decode(&self.refresh_token_key_signing) {
            Ok(value) => value,
            Err(error) => return Err(anyhow::anyhow!(error))
        };

        // Set default values
        let app_name = format!("key-id:{}", self.app_name);
        let footer = Some(app_name.as_str());
        let key = &refresh_token_signing[..];
        let backend = TimeBackend::Chrono;

        // Verify token
        let result = match validate_local_token(token, footer, key, &backend) {
            Ok(value) => value,
            Err(error) => return match error
                .to_string()
                .to_lowercase()
                .as_str() == "this token is expired (exp claim)."
            {
                true => Err(anyhow::anyhow!("Your refresh token has expired")),
                false => Err(anyhow::anyhow!("Invalid refresh token"))
            }
        };

        // Retrieve values from paseto
        let result = result.get("data");
        if result.is_none() {
            return Err(anyhow::anyhow!("Invalid refresh token"));
        }

        // Return value to custom struct
        let result = result.map(|value| value.to_owned()).unwrap_or_default();
        let claims = Claims::from(result);

        // Return claims
        Ok(claims)
    }
}