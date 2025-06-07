use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::test]
pub async fn nodes_reorder() {
    fn nodes_reorder() -> Element {
        let mut data = use_signal(|| vec![1, 2, 3]);

        rsx!(
            rect {
                onclick: move |_| {
                    let item = data.write().remove(0);
                    data.write().push(item);
                },
                label {
                    "Move"
                }
            }
            for d in data.read().iter() {
                label {
                    key: "{d}",
                    height: "20",
                    "{d}"
                }
            }
        )
    }

    let mut utils = launch_test(nodes_reorder);
    utils.wait_for_update().await;

    assert_eq!(utils.root().get(1).get(0).text(), Some("1"));
    assert_eq!(utils.root().get(2).get(0).text(), Some("2"));
    assert_eq!(utils.root().get(3).get(0).text(), Some("3"));

    utils.click_cursor((5., 5.)).await;

    assert_eq!(utils.root().get(1).get(0).text(), Some("2"));
    assert_eq!(utils.root().get(2).get(0).text(), Some("3"));
    assert_eq!(utils.root().get(3).get(0).text(), Some("1"));

    utils.click_cursor((5., 5.)).await;

    assert_eq!(utils.root().get(1).get(0).text(), Some("3"));
    assert_eq!(utils.root().get(2).get(0).text(), Some("1"));
    assert_eq!(utils.root().get(3).get(0).text(), Some("2"));
}
