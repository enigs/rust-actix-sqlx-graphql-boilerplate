use async_graphql::{Context, Result};
use fluent_templates::{ArcLoader, Loader, fluent_bundle::FluentValue};
use std::{collections::HashMap, sync::Arc};
use unic_langid::LanguageIdentifier;

use crate::Errors;
use crate::Locale;
use crate::Response;

impl Locale {
    pub fn new<T, U>(path: T, id: U) -> Self
        where T: ToString,
              U: ToString
    {
        let path = path.to_string();
        let fallback: LanguageIdentifier = id.to_string().parse().expect("Parsing failed...");

        Self {
            loader: ArcLoader::builder(&path, fallback.clone())
                .customize(|bundle| bundle.set_use_isolating(false))
                .build()
                .expect("Failed to build fluent loader..."),
            locales: fallback
        }
    }

    pub fn get(ctx: &Context<'_>) -> Result<Arc<Self>> {
        match ctx.data_opt::<Arc<Self>>() {
            Some(loader) => Ok(Arc::clone(loader)),
            None => Err(Errors::to(
                Response::InternalServerError,
                "Unable to initialize base configuration"
            ))
        }
    }

    pub fn lookup<T>(&self, key: T) -> String
        where T: ToString
    {
        let key = key.to_string();

        self.loader.lookup(&self.locales, &key)
            .unwrap_or(key)
    }

    pub fn lookup_with_args<T, U, V>(&self, key: T, args: &[(U, V)]) -> String
        where T: ToString,
              U: ToString,
              V: ToString
    {
        let key = key.to_string();

        let array: HashMap<String, FluentValue> = args
            .iter()
            .map(|(k, v)| (k.to_string(), FluentValue::from(v.to_string())))
            .collect();

        self.loader.lookup_with_args(&self.locales, &key, &array)
            .unwrap_or(key)
    }
}