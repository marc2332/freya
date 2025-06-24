use freya::prelude::*;
use freya_core::dom::ImageCacheKey;
use freya_testing::prelude::*;

static FREYA_LOGO: &[u8] = include_bytes!("./freya.png");

#[tokio::test]
pub async fn added_removes_images_cache() {
    fn added_removes_images_cache() -> Element {
        let mut data = use_signal(|| 5);
        rsx!(
            rect {
                width: "100",
                height: "100",
                background: "red",
                onclick: move |_| *data.write() -= 1,
            }
            rect {
                width: "100",
                height: "100",
                background: "red",
                onclick: move |_| *data.write() += 1,
            }
            for item in 0..data() {
                image {
                    key: "{item}",
                    image_data: static_bytes(FREYA_LOGO),
                    cache_key: "{item}"
                }
            }
        )
    }

    let mut utils = launch_test(added_removes_images_cache);
    utils.wait_for_update().await;
    let root = utils.root();

    // Simulate a render
    let _ = utils.create_snapshot();

    for (i, _) in root.children_ids()[2..].iter().enumerate() {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert!(images_cache.contains_key(&ImageCacheKey(i.to_string())));
    }

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 5);
    }

    utils.click_cursor((5., 5.)).await;

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 4);
    }

    utils.click_cursor((5., 105.)).await;
    let _ = utils.create_snapshot();

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 5);
    }

    utils.click_cursor((5., 105.)).await;
    let _ = utils.create_snapshot();

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 6);
    }
}

#[tokio::test]
pub async fn changed_image_cache_key() {
    fn changed_image_cache_key() -> Element {
        let mut data = use_signal(|| 1);
        rsx!(
            rect {
                width: "100",
                height: "100",
                background: "red",
                onclick: move |_| *data.write() += 1,
            }
            image {
                image_data: static_bytes(FREYA_LOGO),
                cache_key: "{data()}"
            }
        )
    }

    let mut utils = launch_test(changed_image_cache_key);
    utils.wait_for_update().await;

    // Simulate a render
    let _ = utils.create_snapshot();

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 1);
        assert!(images_cache.contains_key(&ImageCacheKey(1.to_string())));
    }

    utils.click_cursor((5., 5.)).await;
    let _ = utils.create_snapshot();

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 1);
        assert!(images_cache.contains_key(&ImageCacheKey(2.to_string())));
    }

    utils.click_cursor((5., 5.)).await;
    let _ = utils.create_snapshot();

    {
        let sdom = utils.sdom();
        let fdom = sdom.get();
        let images_cache = fdom.images_cache();
        assert_eq!(images_cache.len(), 1);
        assert!(images_cache.contains_key(&ImageCacheKey(3.to_string())));
    }
}
