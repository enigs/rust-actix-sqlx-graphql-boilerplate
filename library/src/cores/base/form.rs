use async_graphql::{Context, MaybeUndefined, InputObject, Result};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

use crate::{Core, Errors, Validator, Response};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::Base, error = "BaseError")]
#[serde(rename_all = "camelCase")]
pub struct BaseForm {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub api_url: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub web_url: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub admin_url: MaybeUndefined<String>,
}

impl BaseForm {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = BaseError {
            api_url: Validator::new(locale, "base-api-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.api_url)
                .validate_string(),
            web_url: Validator::new(locale, "base-web-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.web_url)
                .validate_string(),
            admin_url: Validator::new(locale, "base-admin-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.admin_url)
                .validate_string()
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => Ok(data),
            false => Err(Errors::to(response, error))
        }
    }
}