use app::App;
use dioxus_core::VirtualDom;
use dioxus_native_core::NodeId;
use event_loop::run_event_loop;
use freya_common::EventMessage;
use freya_dom::prelude::SafeDOM;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use winit::event_loop::EventLoopBuilder;

pub use window::WindowEnv;
pub use window_config::WindowConfig;

mod app;
mod elements;
mod event_loop;
mod renderer;
mod window;
mod window_config;
mod wireframe;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

/// Run the app
pub fn run_app<T: 'static + Clone>(
    vdom: VirtualDom,
    rdom: SafeDOM,
    window_config: WindowConfig<T>,
    mutations_notifier: Option<Arc<Notify>>,
    hovered_node: HoveredNode,
) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let _guard = rt.enter();

    let event_loop = EventLoopBuilder::<EventMessage>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // Hotreload
    #[cfg(debug_assertions)]
    {
        use std::process::exit;
        let proxy = proxy.clone();
        dioxus_hot_reload::connect(move |msg| match msg {
            dioxus_hot_reload::HotReloadMsg::UpdateTemplate(template) => {
                let _ = proxy.send_event(EventMessage::UpdateTemplate(template));
            }
            dioxus_hot_reload::HotReloadMsg::Shutdown => exit(0),
        });
    }

    let mut app = App::new(
        rdom,
        vdom,
        &proxy,
        mutations_notifier,
        WindowEnv::from_config(window_config, &event_loop),
    );

    app.init_vdom();
    app.process_layout();

    run_event_loop(app, event_loop, proxy, hovered_node)
}
