use std::path::PathBuf;

use freya_i18n::{
    prelude::{
        I18n,
        I18nConfig,
        use_init_i18n,
    },
    t,
    te,
    tid,
};
use freya_testing::prelude::*;
use unic_langid::{
    LanguageIdentifier,
    langid,
};

#[test]
fn translate_from_static_source() {
    let panic = std::panic::catch_unwind(|| {
        launch_test(|| {
            i18n_from_static();
            t!(&"hello", name: "World")
        })
    });
    assert!(panic.is_ok(), "translate_from_static_source");
}

#[test]
fn failed_to_translate_with_invalid_key() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| t!("invalid"));
        assert!(panic.is_err(), "failed_to_translate_with_invalid_key");
        rect()
    });
}

#[test]
fn failed_to_translate_with_invalid_key_as_error() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| t!("invalid"));
        assert!(
            panic.is_err(),
            "failed_to_translate_with_invalid_key_as_error"
        );
        rect()
    });
}
#[test]
fn failed_to_translate_with_invalid_key_with_args_as_error() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| te!("invalid", name: "<don't care>"));
        assert!(
            panic.is_ok(),
            "failed_to_translate_with_invalid_key_with_args_as_error"
        );
        assert_eq!(
            panic.ok().unwrap().err().unwrap().to_string(),
            "message id not found for key: 'invalid'",
            "failed_to_translate_with_invalid_key_with_args_as_error"
        );
        rect()
    });
}

#[test]
fn failed_to_translate_with_invalid_key_as_id() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("invalid"));
        assert!(panic.is_ok(), "failed_to_translate_with_invalid_key_as_id");
        assert_eq!(
            panic.ok().unwrap(),
            "message id not found for key: 'invalid'".to_string(),
            "failed_to_translate_with_invalid_key_as_id"
        );
        rect()
    });
}

#[test]
fn failed_to_translate_with_invalid_key_with_args_as_id() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("invalid", name: "<don't care>"));
        assert!(
            panic.is_ok(),
            "failed_to_translate_with_invalid_key_with_args_as_id"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "message id not found for key: 'invalid'".to_string(),
            "failed_to_translate_with_invalid_key_with_args_as_id"
        );
        rect()
    });
}

#[test]
fn translate_root_message_in_attributed_definition() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component"));
        assert!(
            panic.is_ok(),
            "translate_root_message_in_attributed_definition"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "My Component".to_string(),
            "translate_root_message_in_attributed_definition"
        );
        rect()
    });
}

#[test]
fn translate_attribute_with_no_args_in_attributed_definition() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component.placeholder"));
        assert!(
            panic.is_ok(),
            "translate_attribute_with_no_args_in_attributed_definition"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "Component's placeholder".to_string(),
            "translate_attribute_with_no_args_in_attributed_definition"
        );
        rect()
    });
}

#[test]
fn translate_attribute_with_args_in_attributed_definition() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component.hint", name: "Zaphod"));
        assert!(
            panic.is_ok(),
            "translate_attribute_with_args_in_attributed_definition"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "Component's hint with parameter \u{2068}Zaphod\u{2069}".to_string(),
            "translate_attribute_with_args_in_attributed_definition"
        );
        rect()
    });
}

#[test]
fn fail_translate_invalid_attribute_with_no_args_in_attributed_definition() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component.not_a_placeholder"));
        assert!(
            panic.is_ok(),
            "fail_translate_invalid_attribute_with_no_args_in_attributed_definition"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "attribute id not found for key: 'my_component.not_a_placeholder'".to_string(),
            "fail_translate_invalid_attribute_with_no_args_in_attributed_definition"
        );
        rect()
    });
}

#[test]
fn fail_translate_invalid_attribute_with_args_in_attributed_definition() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component.not_a_hint", name: "Zaphod"));
        assert!(
            panic.is_ok(),
            "fail_translate_invalid_attribute_with_args_in_attributed_definition"
        );
        assert_eq!(
            panic.ok().unwrap(),
            "attribute id not found for key: 'my_component.not_a_hint'".to_string(),
            "fail_translate_invalid_attribute_with_args_in_attributed_definition"
        );
        rect()
    });
}

#[test]
fn fail_translate_with_invalid_attribute_key() {
    launch_test(|| {
        i18n_from_static();
        let panic = std::panic::catch_unwind(|| tid!("my_component.placeholder.invalid"));
        assert!(panic.is_ok(), "fail_translate_with_invalid_attribute_key");
        assert_eq!(
            panic.ok().unwrap(),
            "invalid message id: 'my_component.placeholder.invalid'".to_string(),
            "fail_translate_with_invalid_attribute_key"
        );
        rect()
    });
}

#[test]
fn translate_from_dynamic_source() {
    launch_test(|| {
        i18n_from_dynamic();
        let panic = std::panic::catch_unwind(|| {
            let name = "World";
            t!("hello", name: name)
        });
        assert!(panic.is_ok(), "translate_from_dynamic_source");
        assert_eq!(
            panic.ok().unwrap(),
            "Hello, \u{2068}World\u{2069}!".to_string(),
            "translate_from_dynamic_source"
        );
        rect()
    });
}

#[test]
#[should_panic]
#[ignore] // Panic hidden within launch_test.
fn fail_translate_from_dynamic_source_when_file_does_not_exist() {
    launch_test(|| {
        i18n_from_dynamic_none_existing();

        rect()
    });
}

#[test]
fn initial_language_is_set() {
    launch_test(|| {
        let value = i18n_from_static();
        assert_eq!(value.language(), EN, "initial_language_is_set");
        rect()
    });
}

#[test]
fn language_can_be_set() {
    launch_test(|| {
        let mut value = i18n_from_static();
        value
            .try_set_language(JP)
            .expect("set_language must succeed");
        assert_eq!(value.language(), JP, "language_can_be_set");
        rect()
    });
}

#[test]
fn no_default_fallback_language() {
    launch_test(|| {
        let value = i18n_from_static();
        assert_eq!(
            format!("{:?}", value.fallback_language()),
            "None".to_string(),
            "no_default_fallback_language"
        );
        rect()
    });
}

#[test]
fn some_default_fallback_language() {
    launch_test(|| {
        let value = i18n_from_static_with_fallback();
        assert_eq!(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"jp\")".to_string(),
            "some_default_fallback_language"
        );
        rect()
    });
}

#[test]
fn fallback_language_can_be_set() {
    launch_test(|| {
        let mut value = i18n_from_static_with_fallback();
        value
            .try_set_fallback_language(EN)
            .expect("try_set_fallback_language must succeed");
        assert_eq!(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"en\")".to_string(),
            "fallback_language_can_be_set"
        );
        rect()
    });
}

#[test]
fn fallback_language_must_have_locale_translation() {
    launch_test(|| {
        let mut value = i18n_from_static_with_fallback();
        let result = value.try_set_fallback_language(IT);
        assert!(
            result.is_err(),
            "fallback_language_must_have_locale_translation"
        );
        assert_eq!(
            result.err().unwrap().to_string(),
            "fallback for \"it\" must have locale".to_string(),
            "fallback_language_must_have_locale_translation"
        );
        assert_eq!(
            format!("{:?}", value.fallback_language().map(|l| l.to_string())),
            "Some(\"jp\")".to_string(),
            "fallback_language_must_have_locale_translation"
        );
        rect()
    });
}

const EN: LanguageIdentifier = langid!("en");
const IT: LanguageIdentifier = langid!("it");
const JP: LanguageIdentifier = langid!("jp");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}

fn i18n_from_static_with_fallback() -> I18n {
    let config = I18nConfig::new(EN)
        .with_locale((EN, include_str!("./data/i18n/en.ftl")))
        .with_fallback(JP);
    use_init_i18n(|| config)
}

fn i18n_from_dynamic() -> I18n {
    let config = I18nConfig::new(EN).with_locale((
        EN,
        PathBuf::from(format!(
            "{}/tests/data/i18n/en.ftl",
            env!("CARGO_MANIFEST_DIR")
        )),
    ));
    use_init_i18n(|| config)
}

fn i18n_from_dynamic_none_existing() -> I18n {
    let config = I18nConfig::new(EN).with_locale((
        EN,
        PathBuf::from(format!(
            "{}/tests/data/i18n/non_existing.ftl",
            env!("CARGO_MANIFEST_DIR")
        )),
    ));
    use_init_i18n(|| config)
}
