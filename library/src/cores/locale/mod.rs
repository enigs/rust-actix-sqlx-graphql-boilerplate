pub mod impls;
pub mod traits;

use fluent_templates::ArcLoader;
use unic_langid::LanguageIdentifier;

pub struct Locale {
    pub loader: ArcLoader,
    pub locales: LanguageIdentifier
}