//! Key translation macros.
//!
//! Using file:
//!
//! ```ftl
//! # en-US.ftl
//! #
//! hello = Hello, {$name}!
//! ```

/// Translate message from key, returning [`crate::prelude::DioxusI18nError`] if id not found...
///
/// ```rust
/// # use freya::prelude::*;
/// # use freya_i18n::prelude::*;
/// # use unic_langid::langid;
/// # fn example() -> impl IntoElement {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let name = "Avery Gigglesworth";
/// let hi = te!("hello", name: {name}).expect("message id 'name' should be present");
/// assert_eq!(hi, "Hello, Avery Gigglesworth");
/// #   rect()
/// # }
/// ```
#[macro_export]
macro_rules! te {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        {
            let mut params_map = $crate::fluent::FluentArgs::new();
            $(
                params_map.set(stringify!($name), $value);
            )*
            $crate::prelude::I18n::get().try_translate_with_args($id, Some(&params_map))
        }
    };

    ($id:expr ) => {{
            $crate::prelude::I18n::get().try_translate($id)
    }};
}

/// Translate message from key, panic! if id not found...
///
/// ```rust
/// # use freya::prelude::*;
/// # use freya_i18n::prelude::*;
/// # use unic_langid::langid;
/// # fn example() -> impl IntoElement {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let name = "Avery Gigglesworth";
/// let hi = t!("hello", name: {name});
/// assert_eq!(hi, "Hello, Avery Gigglesworth");
/// #   rect()
/// # }
/// ```
#[macro_export]
macro_rules! t {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        $crate::te!($id, $( $name : $value ),*).unwrap_or_else(|e| panic!("{}", e.to_string()))
    };

    ($id:expr ) => {{
        $crate::te!($id).unwrap_or_else(|e| panic!("{}", e.to_string()))
    }};
}

/// Translate message from key, return id if no translation found...
///
/// ```rust
/// # use freya::prelude::*;
/// # use freya_i18n::{tid, prelude::*};
/// # use unic_langid::langid;
/// # fn example() -> impl IntoElement {
/// #   let lang = langid!("en-US");
/// #   let config = I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}")).with_fallback(lang.clone());
/// #   let mut i18n = use_init_i18n(|| config);
/// let message = tid!("no-key");
/// assert_eq!(message, "message-id: no-key should be translated");
/// #   rect()
/// # }
/// ```
#[macro_export]
macro_rules! tid {
    ($id:expr, $( $name:ident : $value:expr ),* ) => {
        $crate::te!($id, $( $name : $value ),*).unwrap_or_else(|e| e.to_string())
    };

    ($id:expr ) => {{
        $crate::te!($id).unwrap_or_else(|e| e.to_string())
    }};
}
