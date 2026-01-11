use freya_i18n::prelude::{
    I18n,
    I18nConfig,
    use_init_i18n,
};
use freya_testing::prelude::*;
use unic_langid::{
    LanguageIdentifier,
    langid,
};

#[test]
fn exact_locale_match_will_use_translation() {
    launch_test(|| {
        i18n();

        assert_eq!(
            I18n::get()
                .try_translate("variants")
                .expect("test message id must exist"),
            "variants only".to_string(),
            "exact_locale_match_will_use_translation",
        );

        rect()
    });
}

#[test]
fn non_exact_locale_match_will_use_region() {
    launch_test(|| {
        i18n();

        assert_eq!(
            I18n::get()
                .try_translate("region")
                .expect("test message id must exist"),
            "region only".to_string(),
            "non_exact_locale_match_will_use_region",
        );

        rect()
    });
}

#[test]
fn non_exact_locale_match_will_use_script() {
    launch_test(|| {
        i18n();

        assert_eq!(
            I18n::get()
                .try_translate("script")
                .expect("test message id must exist"),
            "script only".to_string(),
            "non_exact_locale_match_will_use_script",
        );

        rect()
    });
}

#[test]
fn non_exact_locale_match_will_use_language() {
    launch_test(|| {
        i18n();

        assert_eq!(
            I18n::get()
                .try_translate("language")
                .expect("test message id must exist"),
            "language only".to_string(),
            "non_exact_locale_match_will_use_language",
        );

        rect()
    });
}

#[test]
fn no_locale_match_will_use_fallback() {
    launch_test(|| {
        i18n();

        assert_eq!(
            I18n::get()
                .try_translate("fallback")
                .expect("test message id must exist"),
            "fallback only".to_string(),
            "no_locale_match_will_use_fallback",
        );

        rect()
    });
}

fn i18n() -> I18n {
    const FALLBACK_LANG: LanguageIdentifier = langid!("fb-FB");
    const LANGUAGE_LANG: LanguageIdentifier = langid!("la");
    const SCRIPT_LANG: LanguageIdentifier = langid!("la-Scpt");
    const REGION_LANG: LanguageIdentifier = langid!("la-Scpt-LA");
    let variants_lang: LanguageIdentifier = langid!("la-Scpt-LA-variants");

    let config = I18nConfig::new(variants_lang.clone())
        .with_locale((LANGUAGE_LANG, include_str!("../tests/data/fallback/la.ftl")))
        .with_locale((
            SCRIPT_LANG,
            include_str!("../tests/data/fallback/la-Scpt.ftl"),
        ))
        .with_locale((
            REGION_LANG,
            include_str!("../tests/data/fallback/la-Scpt-LA.ftl"),
        ))
        .with_locale((
            variants_lang.clone(),
            include_str!("../tests/data/fallback/la-Scpt-LA-variants.ftl"),
        ))
        .with_locale((
            FALLBACK_LANG,
            include_str!("../tests/data/fallback/fb-FB.ftl"),
        ))
        .with_fallback(FALLBACK_LANG);
    use_init_i18n(|| config)
}
