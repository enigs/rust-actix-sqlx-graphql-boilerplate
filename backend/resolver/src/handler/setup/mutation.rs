use async_graphql::{Context, Object, Result};
use serde_json::json;
use std::sync::Arc;

use library::Core;
use library::Errors;
use library::{Base, BaseForm};
use library::{Mailer, MailerCredentials, MailerForm};
use library::{Paseto, PasetoForm};
use library::{S3, S3Form};

#[derive(Default)]
pub struct SetupMutation;

#[Object]
impl SetupMutation {
    #[autometrics::autometrics]
    async fn base(&self, ctx: &Context<'_>, mut form: BaseForm) -> Result<Base> {
        // Validate form and convert it to Base struct if it's valid
        let form = form.validate(ctx)?
            .to::<Base>();

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert base and update current base
        form.upsert(manager)
            .await
            .map_err(Errors::bad_request)?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.base.try_write() {
                settings.mutate(&form);

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and return error
        let error = Core::locales(ctx)?.lookup("base-update-failed");
        Err(Errors::internal_server_error(error))
    }

    #[autometrics::autometrics]
    async fn mailer(&self, ctx: &Context<'_>, mut form: MailerForm, send_to: String) -> Result<MailerCredentials> {
        // Validate form and convert it to Mailer struct if it's valid
        let mut form = Mailer::from(form.validate(ctx)?
            .to::<MailerCredentials>());

        // Retrieve locale
        let locale = Core::locales(ctx)?;

        // Retrieve mailer variables
        let action = locale.lookup("mailer-action");
        let app_name = config::app_name();
        let service = form.credentials.service.clone().to_uppercase();

        let from = config::MAILER_FROM_NO_REPLY;
        let to = send_to;
        let subject = locale.lookup_with_args(
            "mailer-subject",
            &[("module", form.credentials.service.as_str())]
        );

        // Send email
        form.set_template("emails/setup/config.html")
            .set_context(json!({
                "action": action,
                "app_name": app_name,
                "service": service,
                "web_url": Core::base(ctx)?.get_web_url(),
            }))
            .send(from, to, subject)?;

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert mailer and update current mailer
        form.upsert(manager)
            .await
            .map_err(Errors::bad_request)?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.mailer.try_write() {
                settings.mutate(&form);

                return Ok(settings.credentials.clone());
            }
        }

        // Retrieve locales and return error
        let error = Core::locales(ctx)?.lookup("mailer-update-failed");
        Err(Errors::internal_server_error(error))
    }

    #[autometrics::autometrics]
    async fn paseto(&self, ctx: &Context<'_>, mut form: PasetoForm) -> Result<Paseto> {
        // Validate form and convert it to Paseto struct if it's valid
        let form = form.validate(ctx)?
            .to::<Paseto>();

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert paseto and update current paseto
        form.upsert(manager)
            .await
            .map_err(Errors::bad_request)?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.paseto.try_write() {
                settings.mutate(&form);

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and return error
        let error = Core::locales(ctx)?.lookup("paseto-update-failed");
        Err(Errors::internal_server_error(error))
    }

    #[autometrics::autometrics]
    async fn s3(&self, ctx: &Context<'_>, mut form: S3Form, send_to: String) -> Result<S3> {
        // Validate form and convert it to S3 struct if it's valid
        let form = form.validate(ctx)?
            .to::<S3>();

        // Send test upload
        form.test_image_upload()
            .await?;

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert base and update current base
        form.upsert(manager)
            .await
            .map_err(Errors::bad_request)?;

        // Retrieve locale
        let locale = Core::locales(ctx)?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.s3.try_write() {
                settings.mutate(&form);

                // Retrieve mailer variables
                let action = locale.lookup("s3-action");
                let app_name = config::app_name();

                let from = config::MAILER_FROM_NO_REPLY;
                let to = send_to;
                let subject = locale.lookup_with_args(
                    "mailer-subject",
                    &[("module", "S3")]
                );

                // Send email
                Core::mailer(ctx)?
                    .set_template("emails/setup/config.html")
                    .set_context(json!({
                        "action": action,
                        "app_name": app_name,
                        "service": "S3",
                        "web_url": Core::base(ctx)?.get_web_url(),
                    }))
                    .send(from, to, subject)?;

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and return error
        let error = Core::locales(ctx)?.lookup("s3-update-failed");
        Err(Errors::internal_server_error(error))
    }
}