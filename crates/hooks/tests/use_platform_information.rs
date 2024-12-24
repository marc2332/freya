use dioxus::prelude::*;
use freya::prelude::use_platform_information;
use freya_elements::elements as dioxus_elements;
use freya_testing::prelude::*;

#[tokio::test]
async fn window_size() {
    fn use_animation_app() -> Element {
        let platform_information = use_platform_information();

        let window_size = platform_information.read().viewport_size;

        rsx!(label { "{window_size:?}" })
    }

    let mut utils = launch_test_with_config(
        use_animation_app,
        TestingConfig::<()> {
            size: (333.0, 190.0).into(),
            ..TestingConfig::default()
        },
    );

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("333.0x190.0"));

    utils.resize((500.0, 400.0).into());

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("500.0x400.0"));
}
