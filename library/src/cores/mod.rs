pub mod agent;
pub mod base;
pub mod database;
pub mod locale;
pub mod mailer;
pub mod paseto;
pub mod s3;

use async_graphql::{Context, Result};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use user_agent_parser::UserAgentParser;

use crate::Base;
use crate::DBManager;
use crate::Errors;
use crate::Locale;
use crate::Mailer;
use crate::Paseto;
use crate::Response;
use crate::S3;

/// Core struct - contains core libraries
/// Locales - internationalization for the entire graphql system
/// Base - base settings
/// Mailer - mailer settings & functionalities
/// Paseto - paseto settings & functionalities
/// S3 - s3 settings & functionalities
pub struct Core {
    pub base: Arc<RwLock<Base>>,
    pub database: DBManager,
    pub locale: Arc<Locale>,
    pub mailer: Arc<RwLock<Mailer>>,
    pub paseto: Arc<RwLock<Paseto>>,
    pub s3: Arc<RwLock<S3>>,
    pub user_agent_parser: UserAgentParser
}

impl Core {
    pub async fn init() -> anyhow::Result<Arc<Self>> {
        // Initialize locale
        let locale = Arc::new(Locale::default());

        let user_agent_parser = UserAgentParser::from_path(config::USER_AGENT_REGEXES)
            .unwrap_or_else(|_| { panic!("{}", locale.lookup("user-agent-parser-init-failed")) });

        // Initialize database
        let database = DBManager::init()
            .await?;

        // Migrate database
        sqlx::migrate!("../assets/migrations")
            .run(&database.writer)
            .await?;

        // Initialize base configuration
        let base = Base::init(&database)
            .await?;

        // Initialize mailer configuration
        let mailer = Mailer::init(&database)
            .await?;

        // Initialize paseto configuration
        let paseto = Paseto::init(&database)
            .await?;

        // Initialize s3 configuration
        let s3 = S3::init(&database)
            .await?;

        // Initialize core
        let core = Arc::new(Self {
            base,
            database,
            locale,
            mailer,
            paseto,
            s3,
            user_agent_parser
        });

        // Return core
        Ok(core)
    }

    pub fn locales<'a>(ctx: &Context<'a>) -> Result<&'a Arc<Locale>> {
        let response = Response::InternalServerError;
        let error = "Unable to retrieve locale settings";

        match ctx.data_opt::<Arc<Self>>() {
            Some(settings) => Ok(&settings.locale),
            None => Err(Errors::to(response, error))
        }
    }

    pub fn database<'a>(ctx: &Context<'a>) -> Result<&'a DBManager> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("db-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            return Ok(&settings.database);
        }

        Err(Errors::to(response, error))
    }

    pub fn base<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, Base>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("base-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.base.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn mailer<'a>(ctx: &'a Context<'a>) -> Result<Mailer> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("mailer-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.mailer.try_read() {
                return Ok(settings.clone());
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn paseto<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, Paseto>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("base-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.paseto.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn s3<'a>(ctx: &'a Context<'a>) -> Result<RwLockReadGuard<'a, S3>> {
        let locale = Self::locales(ctx)?;
        let response = Response::InternalServerError;
        let error = locale.lookup("s3-retrieve-failed");

        if let Some(settings) = ctx.data_opt::<Arc<Self>>() {
            if let Ok(settings) = settings.s3.try_read() {
                return Ok(settings);
            }
        }

        Err(Errors::to(response, error))
    }

    pub fn user_agent_parser(&self) -> &UserAgentParser {
        &self.user_agent_parser
    }
}