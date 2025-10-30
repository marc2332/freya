mod error;
pub mod i18n;
pub mod i18n_macro;

pub use fluent;
pub use unic_langid;

pub mod prelude {
    pub use unic_langid::{
        LanguageIdentifier,
        lang,
        langid,
    };

    pub use crate::{
        error::Error as DioxusI18nError,
        i18n::*,
        t,
        te,
        tid,
    };
}
