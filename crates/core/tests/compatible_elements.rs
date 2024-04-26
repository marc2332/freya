use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn compatible_elements() {
    fn compatible_elements_app() -> Element {
        rsx!(
            rect {
                height: "100%",
                width: "100%",
                padding: "10",
                "test",
                label {
                    rect { "test" }
                },
                paragraph {
                    rect { }
                    label { }
                    text { rect {} }
                }
            }
        )
    }

    let mut utils = launch_test(compatible_elements_app);
    utils.wait_for_update().await;
}
