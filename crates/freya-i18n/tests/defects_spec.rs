use freya_i18n::{
    prelude::{
        I18n,
        I18nConfig,
        use_init_i18n,
    },
    t,
};
use freya_testing::*;
use unic_langid::{
    LanguageIdentifier,
    langid,
};

#[test]
fn issue_15_recent_change_to_t_macro_unnecessarily_breaks_v0_3_code_test_attr() {
    let panic = std::panic::catch_unwind(|| {
        launch_test(|| {
            i18n_from_static();
            t!(&"hello", name: "World")
        })
    });
    assert!(panic.is_ok(), "translate_from_static_source");
}

const EN: LanguageIdentifier = langid!("en");

fn i18n_from_static() -> I18n {
    let config = I18nConfig::new(EN).with_locale((EN, include_str!("./data/i18n/en.ftl")));
    use_init_i18n(|| config)
}
