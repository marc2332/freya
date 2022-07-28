use dioxus::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    launch(app);
}

pub fn launch(app: Component<()>) {
    let mut dom = VirtualDom::new(app);

    let _inital_edits = dom.rebuild();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                thread::sleep(Duration::from_millis(100));
                println!("render loop");
                let mutations = dom.work_with_deadline(|| false);
                println!("{:?}", mutations);
                dom.wait_for_work().await;
            }
        });
}

fn app(cx: Scope) -> Element {
    let v = use_state(&cx, || 0);

    use_effect(&cx, (v,), |v| async move {
        v.0.modify(|v| v + 1);
    });

    cx.render(rsx! {
        div {
            h1 {
                "-> {v}"
            },
            "{v}"
        }
    })
}
