use freya::prelude::*;
use freya_testing::prelude::*;

#[tokio::main]
async fn main() {
    println!("cargo::rerun-if-changed=../components/src/*");

    let mut utils = launch_test(|| {
        rsx!(
            rect {
                main_align: "center",
                cross_align: "center",
                width: "fill",
                height: "fill",
                Button {
                    label {
                        "Click this"
                    }
                }
            }
        )
    });

    utils.resize((200., 150.).into());

    utils.wait_for_update().await;

    utils.save_snapshot("../components/images/Button.png");
}
