use freya::prelude::*;
use freya_testing::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn asset_cacher() {
    #[allow(non_snake_case)]
    fn Consumer() -> Element {
        let mut cacher = use_asset_cacher();

        let asset = use_hook(move || {
            let asset_config = AssetConfiguration {
                age: Duration::from_millis(1).into(),
                id: "test-asset".to_string(),
            };
            cacher.use_asset(&asset_config).unwrap()
        });

        use_drop(move || {
            let asset_config = AssetConfiguration {
                age: Duration::from_millis(1).into(),
                id: "test-asset".to_string(),
            };
            cacher.unuse_asset(asset_config.clone());
        });

        rsx!(label { "{asset.read()[2]}" })
    }

    fn asset_cacher_app() -> Element {
        let mut consumers = use_signal(|| 0);
        let mut cacher = use_asset_cacher();

        use_hook(move || {
            let asset_config = AssetConfiguration {
                age: Duration::from_millis(1).into(),
                id: "test-asset".to_string(),
            };

            cacher.cache(asset_config.clone(), vec![9, 8, 7, 6].into(), false);
        });

        rsx!(
            label {
                "size {cacher.size()}"
            }
            Button {
                onclick: move |_| consumers += 1 ,
                label {
                    "add"
                }
            }
            Button {
                onclick: move |_| consumers -= 1 ,
                label {
                    "remove"
                }
            }
            for i in 0..consumers() {
                Consumer {
                    key: "{i}"
                }
            }
        )
    }

    let mut utils = launch_test(asset_cacher_app);

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("size 1"));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 25.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("size 1"));
    assert_eq!(utils.root().get(3).get(0).text(), Some("7"));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 25.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("size 1"));
    assert_eq!(utils.root().get(4).get(0).text(), Some("7"));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 70.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("size 1"));

    utils.push_event(PlatformEvent::Mouse {
        name: EventName::Click,
        cursor: (5.0, 70.0).into(),
        button: Some(MouseButton::Left),
    });

    utils.wait_for_update().await;
    sleep(Duration::from_millis(5)).await;
    utils.wait_for_update().await;
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(0).get(0).text(), Some("size 0"));
}
