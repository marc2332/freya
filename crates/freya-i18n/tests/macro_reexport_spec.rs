// Test to verify that macros can be re-exported from other crates
// This ensures the fix for hard-coded crate names is working properly

mod reexport_test {
    // Re-export the macros as if from another crate
    pub use freya_i18n::{
        t,
        te,
        tid,
    };
}

use freya_i18n::prelude::{
    I18nConfig,
    use_init_i18n,
};
use freya_testing::*;
use unic_langid::langid;

#[test]
fn reexported_te_macro_works() {
    launch_test(|| {
        let lang = langid!("en");
        let config =
            I18nConfig::new(lang.clone()).with_locale((lang.clone(), "hello = Hello, {$name}!"));
        let _i18n = use_init_i18n(|| config);

        // Use the re-exported macro
        let result = reexport_test::te!("hello", name: "World");
        assert!(result.is_ok(), "reexported te! macro should work");
        assert_eq!(result.unwrap(), "Hello, \u{2068}World\u{2069}!");

        rect()
    });
}

#[test]
fn reexported_t_macro_works() {
    launch_test(|| {
        let lang = langid!("en");
        let config =
            I18nConfig::new(lang.clone()).with_locale((lang.clone(), "greeting = Hi there!"));
        let _i18n = use_init_i18n(|| config);

        // Use the re-exported macro
        let result = reexport_test::t!("greeting");
        assert_eq!(result, "Hi there!");

        rect()
    });
}

#[test]
fn reexported_tid_macro_works() {
    launch_test(|| {
        let lang = langid!("en");
        let config =
            I18nConfig::new(lang.clone()).with_locale((lang.clone(), "message = Test message"));
        let _i18n = use_init_i18n(|| config);

        // Use the re-exported macro with valid key
        let result = reexport_test::tid!("message");
        assert_eq!(result, "Test message");

        // Use the re-exported macro with invalid key
        let result_invalid = reexport_test::tid!("invalid-key");
        assert!(result_invalid.contains("message id not found"));

        rect()
    });
}
