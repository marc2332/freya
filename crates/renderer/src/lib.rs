use app::App;
pub use config::*;
use devtools::Devtools;
use dioxus_core::VirtualDom;
use freya_common::EventMessage;
use freya_core::dom::SafeDOM;
use freya_native_core::NodeId;
use std::sync::{Arc, Mutex};
use winit::event_loop::EventLoopBuilder;

pub use config::WindowConfig;
pub use window::WindowEnv;

mod accessibility;
mod app;
mod config;
pub mod devtools;
mod elements;
mod event_loop;
mod renderer;
mod window;
mod winit_waker;
mod wireframe;

pub type HoveredNode = Option<Arc<Mutex<Option<NodeId>>>>;

/// Desktop renderer for Freya.
pub struct DesktopRenderer;

impl DesktopRenderer {
    pub fn launch<T: 'static + Clone>(
        vdom: VirtualDom,
        sdom: SafeDOM,
        config: LaunchConfig<T>,
        devtools: Option<Devtools>,
        hovered_node: HoveredNode,
    ) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let _guard = rt.enter();

        let event_loop = EventLoopBuilder::<EventMessage>::with_user_event()
            .build()
            .expect("Failed to create event loop.");
        let proxy = event_loop.create_proxy();

        // Hotreload support for Dioxus
        #[cfg(feature = "hot-reload")]
        {
            use std::process::exit;
            let proxy = proxy.clone();
            dioxus_hot_reload::connect(move |msg| match msg {
                dioxus_hot_reload::HotReloadMsg::UpdateTemplate(template) => {
                    let _ = proxy.send_event(EventMessage::UpdateTemplate(template));
                }
                dioxus_hot_reload::HotReloadMsg::Shutdown => exit(0),
                dioxus_hot_reload::HotReloadMsg::UpdateAsset(_) => {}
            });
        }

        let window_env = WindowEnv::new(config.window, &event_loop);

        let mut app = App::new(
            sdom,
            vdom,
            &proxy,
            devtools,
            window_env,
            config.embedded_fonts,
            config.plugins,
            config.default_fonts,
        );

        app.init_doms();
        app.process_layout();
        app.run(event_loop, proxy, hovered_node)
    }
}
