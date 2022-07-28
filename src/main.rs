use dioxus::prelude::*;
use dioxus_native_core::real_dom::{self, RealDom};
use dioxus_native_core::state::State;
use dioxus_native_core_macro::State;
use gl::*;
use glutin::*;
use skia_safe::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
mod run;

fn main() {
    launch(app);
}

#[derive(Debug, Clone, State, Default)]
pub struct Lala {}

pub fn launch(app: Component<()>) {
    let rdom = Arc::new(Mutex::new(RealDom::<Lala>::new()));

    {
        let rdom = rdom.clone();
        std::thread::spawn(move || {
            let mut dom = VirtualDom::new(app);

            let muts = dom.rebuild();
            rdom.lock().unwrap().apply_mutations(vec![muts]);
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
                        rdom.lock().unwrap().apply_mutations(mutations);
                        dom.wait_for_work().await;
                    }
                });
        });
    }

    run::run(rdom.clone());
}

fn app(cx: Scope) -> Element {

    cx.render(rsx! {
        div {

        }
    })
}
