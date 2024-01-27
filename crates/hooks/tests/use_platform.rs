use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_animation, use_platform, Animation, PlatformInformation};
use freya_testing::{launch_test, launch_test_with_config, TestingConfig};
use torin::geometry::Size2D;

#[tokio::test]
async fn window_size() {
    fn use_animation_app() -> Element {
        let mut animation = use_animation(|| 0.0);

        let progress = animation.value();

        let _ = use_memo(move || {
            animation.start(Animation::new_linear(0.0..=100.0, 50));
        });

        rsx!(rect {
            width: "{progress}",
        })
    }

    let mut utils = launch_test(use_animation_app);

    utils.wait_for_update().await;

    utils.root().get(0).layout().clone().unwrap();
}
