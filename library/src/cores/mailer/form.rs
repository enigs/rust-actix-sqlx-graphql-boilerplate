use async_graphql::{Context, MaybeUndefined, InputObject, Result};
use serde::{Serialize, Deserialize};

use macros::{AsForm, SetIsEmpty};

use crate::{Core, Errors, Validator, Response};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, InputObject)]
#[derive(AsForm, SetIsEmpty)]
#[form(to = crate::MailerCredentials, error = "MailerError")]
#[serde(rename_all = "camelCase")]
pub struct MailerForm {
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub username: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub password: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub smtp_host: MaybeUndefined<String>,
    #[serde(skip_serializing_if = "MaybeUndefined::is_undefined")]
    #[conversion(crate::conversions::mustr2str)]
    #[sanitize(crate::sanitize::mustring)]
    #[error(String)]
    pub service: MaybeUndefined<String>,
}

impl MailerForm {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = MailerError {
            username: Validator::new(locale, "mailer-username")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.username)
                .validate_string(),
            password: Validator::new(locale, "mailer-password")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.password)
                .validate_string(),
            smtp_host: Validator::new(locale, "mailer-smtp-host")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.smtp_host)
                .validate_string(),
            service: Validator::new(locale, "mailer-service")
                .set_option_list_string(&["SES", "MAILGUN"])
                .set_as_case_sensitive(false)
                .set_as_required(true)
                .set_string_value(&data.service)
                .validate_list_string()
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => Ok(data),
            false => Err(Errors::to(response, error))
        }
    }
}