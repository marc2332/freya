use std::{
    collections::HashMap,
    path::PathBuf,
};

use freya_i18n::prelude::*;
use unic_langid::langid;

#[test]
fn can_add_locale_to_config_explicit_locale() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_B: LanguageIdentifier = langid!("la-LB");
    const LANG_C: LanguageIdentifier = langid!("la-LC");

    let config = I18nConfig::new(LANG_A)
        .with_locale(Locale::new_static(LANG_B, "lang = lang_b"))
        .with_locale(Locale::new_dynamic(LANG_C, PathBuf::new()));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![
                LocaleResource::Static("lang = lang_b"),
                LocaleResource::Path(PathBuf::new()),
            ],
            locales: HashMap::from([(LANG_B, 0), (LANG_C, 1)]),
        }
    );
}

#[test]
fn can_add_locale_to_config_implicit_locale() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_B: LanguageIdentifier = langid!("la-LB");
    const LANG_C: LanguageIdentifier = langid!("la-LC");

    let config = I18nConfig::new(LANG_A)
        .with_locale((LANG_B, "lang = lang_b"))
        .with_locale((LANG_C, PathBuf::new()));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![
                LocaleResource::Static("lang = lang_b"),
                LocaleResource::Path(PathBuf::new())
            ],
            locales: HashMap::from([(LANG_B, 0), (LANG_C, 1)]),
        }
    );
}

#[test]
fn can_add_locale_string_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_B: LanguageIdentifier = langid!("la-LB");

    let config = I18nConfig::new(LANG_A).with_locale((LANG_B, "lang = lang_b"));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![LocaleResource::Static("lang = lang_b")],
            locales: HashMap::from([(LANG_B, 0)]),
        }
    );
}

#[test]
fn can_add_shared_locale_string_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_B: LanguageIdentifier = langid!("la-LB");
    const LANG_C: LanguageIdentifier = langid!("la-LC");

    let shared_string = "lang = a language";
    let config = I18nConfig::new(LANG_A)
        .with_locale((LANG_B, shared_string))
        .with_locale((LANG_C, shared_string));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![LocaleResource::Static(shared_string)],
            locales: HashMap::from([(LANG_B, 0), (LANG_C, 0)]),
        }
    );
}

#[test]
fn can_add_locale_pathbuf_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_C: LanguageIdentifier = langid!("la-LC");

    let config =
        I18nConfig::new(LANG_A).with_locale((LANG_C, PathBuf::from("./test/data/fallback/la.ftl")));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![LocaleResource::Path(PathBuf::from(
                "./test/data/fallback/la.ftl"
            ))],
            locales: HashMap::from([(LANG_C, 0)]),
        }
    );
}

#[test]
fn can_add_shared_locale_pathbuf_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");
    const LANG_B: LanguageIdentifier = langid!("la-LB");
    const LANG_C: LanguageIdentifier = langid!("la-LC");

    let shared_pathbuf = PathBuf::from("./test/data/fallback/la.ftl");

    let config = I18nConfig::new(LANG_A)
        .with_locale((LANG_B, shared_pathbuf.clone()))
        .with_locale((LANG_C, shared_pathbuf.clone()));

    assert_eq!(
        config,
        I18nConfig {
            id: LANG_A,
            fallback: None,
            locale_resources: vec![LocaleResource::Path(shared_pathbuf)],
            locales: HashMap::from([(LANG_B, 0), (LANG_C, 0)]),
        }
    );
}

#[cfg(feature = "discovery")]
#[test]
fn can_auto_add_locales_folder_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");

    let root_path_str = &format!("{}/tests/data/fallback/", env!("CARGO_MANIFEST_DIR"));
    let pathbuf = PathBuf::from(root_path_str);

    let config = I18nConfig::new(LANG_A)
        .try_with_auto_locales(pathbuf)
        .ok()
        .unwrap();

    let expected_locales = [
        "fb-FB",
        "la",
        "la-Scpt",
        "la-Scpt-LA",
        "la-Scpt-LA-variants",
    ];

    assert_eq!(config.locales.len(), expected_locales.len());
    assert_eq!(config.locale_resources.len(), expected_locales.len());

    expected_locales.into_iter().for_each(|l| {
        let expected_filename = format!("{root_path_str}/{l}.ftl");
        let id = LanguageIdentifier::from_bytes(l.as_bytes()).unwrap();
        assert!(config.locales.contains_key(&id));
        assert!(
            config
                .locale_resources
                .contains(&LocaleResource::Path(PathBuf::from(expected_filename)))
        );
    });
}

#[cfg(feature = "discovery")]
#[test]
fn can_auto_add_locales_file_to_config() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");

    let path_str = &format!(
        "{}/tests/data/fallback/fb-FB.ftl",
        env!("CARGO_MANIFEST_DIR")
    );
    let pathbuf = PathBuf::from(path_str);

    let config = I18nConfig::new(LANG_A)
        .try_with_auto_locales(pathbuf.clone())
        .ok()
        .unwrap();

    assert_eq!(config.locales.len(), 1);
    assert!(config.locales.contains_key(&langid!("fb-FB")));

    assert_eq!(config.locale_resources.len(), 1);
    assert!(
        config
            .locale_resources
            .contains(&LocaleResource::Path(pathbuf))
    );
}

#[cfg(feature = "discovery")]
#[test]
fn will_fail_auto_locales_with_invalid_folder() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");

    let root_path_str = &format!("{}/non_existing_path/", env!("CARGO_MANIFEST_DIR"));
    let pathbuf = PathBuf::from(root_path_str);

    let config = I18nConfig::new(LANG_A).try_with_auto_locales(pathbuf);
    assert!(config.is_err());
}

#[cfg(feature = "discovery")]
#[test]
fn will_fail_auto_locales_with_invalid_file() {
    const LANG_A: LanguageIdentifier = langid!("la-LA");

    let path_str = &format!(
        "{}/tests/data/fallback/invalid_language_id.ftl",
        env!("CARGO_MANIFEST_DIR")
    );
    let pathbuf = PathBuf::from(path_str);

    let config = I18nConfig::new(LANG_A).try_with_auto_locales(pathbuf);
    assert!(config.is_err());
}
