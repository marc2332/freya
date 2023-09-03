use app::App;
pub use config::*;
use dioxus_core::VirtualDom;
use dioxus_native_core::NodeId;
use event_loop::run_event_loop;
use freya_common::EventMessage;
use freya_dom::prelude::SafeDOM;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use winit::event_loop::EventLoopBuilder;

pub use config::WindowConfig;
pub use window::WindowEnv;

mod accessibility;
mod app;
mod config;
mod elements;
mod event_loop;
mod renderer;
mod window;
mod wireframe;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

/// Run the app
pub fn run_app<T: 'static + Clone>(
    vdom: VirtualDom,
    rdom: SafeDOM,
    config: LaunchConfig<T>,
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

    let window_env = WindowEnv::from_config(config.window.clone(), &event_loop);

    let mut app = App::new(rdom, vdom, &proxy, mutations_notifier, window_env, config);

    app.init_vdom();
    app.process_layout();

    run_event_loop(app, event_loop, proxy, hovered_node)
}
