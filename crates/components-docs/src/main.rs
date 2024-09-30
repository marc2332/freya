#[rustfmt::skip]
#[tokio::main]
async fn main(){
    {
        mod preview {
            use freya_testing::prelude::*;

            pub async fn run(){
                use freya::prelude::*;
                fn app() -> Element {
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
                }

                let mut utils = launch_test(app);
                utils.resize((200.,150.).into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/button.png");
            }
        }

        preview::run().await;
    }

    {
        mod preview {
            use freya_testing::prelude::*;

            pub async fn run(){
                use freya::prelude::*;
                fn app() -> Element {
                    rsx!(
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "fill",
                            height: "fill",
                            Switch {
                                enabled: false,
                                ontoggled: move |_| { }
                            }
                        }
                    )
                }

                let mut utils = launch_test(app);
                utils.resize((200.,150.).into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/gallery_enabled_switch.png");
            }
        }

        preview::run().await;
    }

    {
        mod preview {
            use freya_testing::prelude::*;

            pub async fn run(){
                use freya::prelude::*;
                fn app() -> Element {
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
                }

                let mut utils = launch_test(app);
                utils.resize((200.,150.).into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/gallery_button.png");
            }
        }

        preview::run().await;
    }

    {
        mod preview {
            use freya_testing::prelude::*;

            pub async fn run(){
                use freya::prelude::*;
                fn app() -> Element {
                    rsx!(
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "fill",
                            height: "fill",
                            Switch {
                                enabled: true,
                                ontoggled: move |_| { }
                            }
                        }
                    )
                }

                let mut utils = launch_test(app);
                utils.resize((200.,150.).into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/gallery_not_enabled_switch.png");
            }
        }

        preview::run().await;
    }

    {
        mod preview {
            use freya_testing::prelude::*;

            pub async fn run(){
                use freya::prelude::*;
                fn app() -> Element {
                    rsx!(
                        rect {
                            main_align: "center",
                            cross_align: "center",
                            width: "fill",
                            height: "fill",
                            Switch {
                                enabled: false,
                                ontoggled: move |_| { }
                            }
                        }
                    )
                }

                let mut utils = launch_test(app);
                utils.resize((200.,150.).into());
                utils.wait_for_update().await;
                utils.save_snapshot("./crates/components/images/gallery_enabled_switch.png");
            }
        }

        preview::run().await;
    }
}