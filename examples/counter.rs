#![allow(non_snake_case)]
use dioxus::prelude::dioxus_core::NoOpMutations;
use freya::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fn app() -> Element {
        rsx! {
            Spammer {}
            Spammer {}
            Spammer {}
            Spammer {}
        }
    }

    fn Spammer() -> Element {
        let mut count = use_signal(|| 0);

        use_hook(|| {
            spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
                    let val = *count.peek_unchecked();
                    if val == 70 {
                        count.set(0);
                    } else {
                        count.set(val + 1);
                    }
                }
            })
        });

        rsx! {
            for el in 0..*count.read() {
                // To avoid the leak simply replace this by `div {`
                Comp {
                    key: "{el}",
                }
            }
        }
    }

    #[component]
    fn Comp() -> Element {
        rsx!(rect {})
    }

    // create the vdom, the real_dom, and the binding layer between them
    let mut vdom = VirtualDom::new(app);

    vdom.rebuild(&mut NoOpMutations);

    // we need to run the vdom in a async runtime
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            loop {
                // wait for the vdom to update
                vdom.wait_for_work().await;

                // get the mutations from the vdom
                vdom.render_immediate(&mut NoOpMutations);
            }
        })
}
