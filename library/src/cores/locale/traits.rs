use crate::Locale;

impl Default for Locale {
    fn default() -> Self {
        Locale::new(config::LOCALES_PATH, config::LOCALES_US)
    }
}