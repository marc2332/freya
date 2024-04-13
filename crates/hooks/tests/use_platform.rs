use dioxus::prelude::*;
use freya_hooks::use_platform;
use freya_testing::prelude::*;

#[tokio::test]
async fn window_size() {
    fn use_animation_app() -> Element {
        let platform = use_platform();

        let platform = platform.info().window_size;

        rsx!(label { "{platform:?}" })
    }

    let mut utils = launch_test_with_config(
        use_animation_app,
        TestingConfig {
            size: (333.0, 190.0).into(),
            ..TestingConfig::default()
        },
    );

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("333.0x190.0"));
}
