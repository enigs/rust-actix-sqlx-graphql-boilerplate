use async_graphql::MaybeUndefined;
use std::sync::Arc;

use crate::Locale;

pub mod validate;

#[derive(Clone, Default)]
pub struct Validator {
    pub locales: Arc<Locale>,
    pub field: String,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub option_list_string: Option<Vec<String>>,
    pub is_case_sensitive: bool,
    pub is_nullable: bool,
    pub is_required: bool,
    pub i32_value: i32,
    pub i64_value: i64,
    pub string_value: String,
}

impl Validator {
    pub fn new<T>(locale: &Arc<Locale>, field: T) -> Self
        where T: ToString
    {
        Validator {
            locales: Arc::clone(locale),
            field: field.to_string(),
            is_required: false,
            is_nullable: false,
            ..Default::default()
        }
    }

    pub fn set_as_case_sensitive(&mut self, is_case_sensitive: bool) -> &mut Self {
        self.is_case_sensitive = is_case_sensitive;
        self
    }

    pub fn set_as_nullable(&mut self, is_nullable: bool) -> &mut Self {
        self.is_nullable = is_nullable;
        self
    }

    pub fn set_as_required(&mut self, is_required: bool) -> &mut Self {
        self.is_required = is_required;
        self
    }

    pub fn set_min(&mut self, min: usize) -> &mut Self {
        self.min = Some(min);
        self
    }

    pub fn set_max(&mut self, max: usize) -> &mut Self {
        self.max = Some(max);
        self
    }

    pub fn set_option_list_string<T>(&mut self, option_list_string: &[T]) -> &mut Self
        where T: ToString
    {
        self.option_list_string = Some(option_list_string
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>());

        self
    }

    pub fn set_i32_value(&mut self, int32: &MaybeUndefined<i32>) -> &mut Self {
        self.i32_value = int32.take().unwrap_or_default();
        self
    }

    pub fn set_i64_value(&mut self, int64: &MaybeUndefined<i64>) -> &mut Self {
        self.i64_value = int64.take().unwrap_or_default();
        self
    }

    pub fn set_string_value(&mut self, string: &MaybeUndefined<String>) -> &mut Self {
        self.string_value = string.clone().take().unwrap_or_default().to_string();
        self
    }
}