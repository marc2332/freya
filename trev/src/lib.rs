use anymap::AnyMap;
use dioxus::prelude::*;
use dioxus_native_core::real_dom::RealDom;
use renderer::run;
use state::node::NodeState;
use std::sync::Mutex;
use std::sync::{mpsc, Arc};

pub use renderer;

pub fn launch(app: Component<()>) {
    let rdom = Arc::new(Mutex::new(RealDom::<NodeState>::new()));
    let (trig_render, rev_render) = mpsc::channel::<()>();

    {
        let rdom = rdom.clone();
        std::thread::spawn(move || {
            let mut dom = VirtualDom::new(app);

            let muts = dom.rebuild();
            let to_update = rdom.lock().unwrap().apply_mutations(vec![muts]);
            let mut ctx = AnyMap::new();
            ctx.insert(0.0f32);
            rdom.lock().unwrap().update_state(&dom, to_update, ctx);
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async move {
                    loop {
                        dom.wait_for_work().await;
                        let mutations = dom.work_with_deadline(|| false);
                        let to_update = rdom.lock().unwrap().apply_mutations(mutations);
                        let ctx = AnyMap::new();
                        if !to_update.is_empty() {
                            trig_render.send(()).unwrap();
                        }
                        rdom.lock().unwrap().update_state(&dom, to_update, ctx);
                    }
                });
        });
    }

    run(rdom, rev_render);
}
