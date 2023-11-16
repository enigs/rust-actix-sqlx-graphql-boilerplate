use async_graphql::{Context, Object, Result};

use library::Core;
use library::Base;
use library::MailerCredentials;
use library::Paseto;
use library::S3;

#[derive(Default)]
pub struct SetupQuery;

#[Object]
impl SetupQuery {
    #[autometrics::autometrics]
    async fn base(&self, ctx: &Context<'_>) -> Result<Base> {
        Ok(Core::base(ctx)?.clone())
    }

    #[autometrics::autometrics]
    async fn mailer(&self, ctx: &Context<'_>) -> Result<MailerCredentials> {
        Ok(Core::mailer(ctx)?.clone().credentials)
    }

    #[autometrics::autometrics]
    async fn paseto(&self, ctx: &Context<'_>) -> Result<Paseto> {
        Ok(Core::paseto(ctx)?.clone())
    }

    #[autometrics::autometrics]
    async fn s3(&self, ctx: &Context<'_>) -> Result<S3> {
        Ok(Core::s3(ctx)?.clone())
    }
}