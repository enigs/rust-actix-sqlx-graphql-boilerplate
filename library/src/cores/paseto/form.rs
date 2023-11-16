use async_graphql::{Context, MaybeUndefined, InputObject, Result};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

use crate::Paseto;
use crate::{Core, Errors, Validator, Response};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::Paseto, error = "PasetoError")]
#[serde(rename_all = "camelCase")]
pub struct PasetoForm {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub app_name: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui642str)]
    #[error(String)]
    pub access_token_key_unit: MaybeUndefined<i64>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_token_key_time: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub access_token_key_signing: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mui642str)]
    #[error(String)]
    pub refresh_token_key_unit: MaybeUndefined<i64>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub refresh_token_key_time: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub refresh_token_key_signing: MaybeUndefined<String>,
}

impl PasetoForm {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = PasetoError {
            app_name: Validator::new(locale, "paseto-app-name")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.app_name)
                .validate_string(),
            access_token_key_unit: Validator::new(locale, "paseto-access-token-key-unit")
                .set_min(1)
                .set_max(1000)
                .set_as_required(true)
                .set_i64_value(&data.access_token_key_unit)
                .validate_i64(),
            access_token_key_time: Validator::new(locale, "paseto-access-token-key-time")
                .set_as_required(true)
                .set_option_list_string(&["seconds", "minutes", "hours", "days", "weeks", "months"])
                .set_as_case_sensitive(false)
                .set_string_value(&data.access_token_key_time)
                .validate_list_string(),
            access_token_key_signing: Validator::new(locale, "paseto-access-token-key-signing")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.access_token_key_signing)
                .validate_string(),
            refresh_token_key_unit: Validator::new(locale, "paseto-refresh-token-key-unit")
                .set_min(1)
                .set_max(1000)
                .set_as_required(true)
                .set_i64_value(&data.refresh_token_key_unit)
                .validate_i64(),
            refresh_token_key_time: Validator::new(locale, "paseto-refresh-token-key-time")
                .set_as_required(true)
                .set_option_list_string(&["seconds", "minutes", "hours", "days", "weeks", "months"])
                .set_as_case_sensitive(false)
                .set_string_value(&data.refresh_token_key_time)
                .validate_list_string(),
            refresh_token_key_signing: Validator::new(locale, "paseto-refresh-token-key-signing")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.refresh_token_key_signing)
                .validate_string()
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => {
                let access_duration = Paseto::get_duration(
                    data.access_token_key_unit.take().unwrap_or(0),
                    data.access_token_key_time.clone().take().unwrap_or_default(),
                    data.access_token_key_unit.take().unwrap_or(0)
                );

                let refresh_duration = Paseto::get_duration(
                    data.refresh_token_key_unit.take().unwrap_or(0),
                    data.refresh_token_key_time.clone().take().unwrap_or_default(),
                    data.refresh_token_key_unit.take().unwrap_or(0)
                );

                if access_duration > refresh_duration {
                    let error = locale.lookup("paseto-token-key-unit-invalid");
                    return Err(Errors::to(response, error));
                }

                Ok(data)
            },
            false => Err(Errors::to(response, error))
        }
    }
}