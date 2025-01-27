use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn auto_sized_center_text() {
    fn auto_sized_center_text_app() -> Element {
        rsx!(
            label {
                font_size: "18",
                text_align: "center",
                width: "auto",
                "Center align with auto size\nNew line\nLast line"
            }
        )
    }

    let mut utils = launch_test(auto_sized_center_text_app);
    utils.wait_for_update().await;

    let root = utils.root();
    let width = root.get(0).layout().unwrap().area.width();
    assert!(width > 200. && width < 220.);
}
