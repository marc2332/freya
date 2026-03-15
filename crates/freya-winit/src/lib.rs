pub mod reexports {
    pub use winit;
}

use std::sync::Arc;

use crate::{
    config::LaunchConfig,
    renderer::{
        LaunchProxy,
        NativeEvent,
        NativeGenericEvent,
        WinitRenderer,
    },
};
mod accessibility;
pub mod config;
mod drivers;
pub mod extensions;
pub mod plugins;
pub mod renderer;
#[cfg(feature = "tray")]
mod tray_icon;
mod window;
mod winit_mappings;

pub use extensions::*;
use futures_util::task::{
    ArcWake,
    waker,
};

use crate::winit::event_loop::EventLoopProxy;

pub mod winit {
    pub use winit::*;
}

#[cfg(feature = "tray")]
pub mod tray {
    pub use tray_icon::*;

    pub use crate::tray_icon::*;
}

/// Launch the application.
pub fn launch(launch_config: LaunchConfig) {
    use winit::event_loop::EventLoop;

    setup_panic_hook();

    let event_loop = EventLoop::<NativeEvent>::with_user_event()
        .build()
        .expect("Failed to create event loop.");

    launch_inner(launch_config, event_loop);
}

/// Launch the application with access to the [`EventLoopBuilder`](winit::event_loop::EventLoopBuilder)
/// and the constructed [`EventLoop`](winit::event_loop::EventLoop).
pub fn launch_with_event_loop(
    builder_hook: impl FnOnce(&mut winit::event_loop::EventLoopBuilder<NativeEvent>),
    setup: impl FnOnce(&winit::event_loop::EventLoop<NativeEvent>) -> LaunchConfig,
) {
    use winit::event_loop::EventLoop;

    setup_panic_hook();

    let mut event_loop_builder = EventLoop::<NativeEvent>::with_user_event();
    builder_hook(&mut event_loop_builder);
    let event_loop = event_loop_builder
        .build()
        .expect("Failed to create event loop.");

    let launch_config = setup(&event_loop);

    launch_inner(launch_config, event_loop);
}

fn setup_panic_hook() {
    #[cfg(all(not(debug_assertions), not(target_os = "android")))]
    {
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            rfd::MessageDialog::new()
                .set_title("Fatal Error")
                .set_description(&panic_info.to_string())
                .set_level(rfd::MessageLevel::Error)
                .show();
            previous_hook(panic_info);
            std::process::exit(1);
        }));
    }
}

fn launch_inner(
    launch_config: LaunchConfig,
    event_loop: winit::event_loop::EventLoop<NativeEvent>,
) {
    use std::collections::HashMap;

    use freya_core::integration::*;
    use freya_engine::prelude::{
        FontCollection,
        FontMgr,
        TypefaceFontProvider,
    };

    let proxy = event_loop.create_proxy();

    let mut font_collection = FontCollection::new();
    let def_mgr = FontMgr::default();
    let mut provider = TypefaceFontProvider::new();
    for (font_name, font_data) in launch_config.embedded_fonts {
        let ft_type = def_mgr
            .new_from_data(&font_data, None)
            .unwrap_or_else(|| panic!("Failed to load font {font_name}."));
        provider.register_typeface(ft_type, Some(font_name.as_ref()));
    }
    let font_mgr: FontMgr = provider.into();
    font_collection.set_default_font_manager(def_mgr, None);
    font_collection.set_dynamic_font_manager(font_mgr.clone());
    font_collection.paragraph_cache_mut().turn_on(false);

    let screen_reader = ScreenReader::new();

    struct FuturesWaker(EventLoopProxy<NativeEvent>);

    impl ArcWake for FuturesWaker {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            _ = arc_self
                .0
                .send_event(NativeEvent::Generic(NativeGenericEvent::PollFutures));
        }
    }

    let waker = waker(Arc::new(FuturesWaker(proxy.clone())));

    let mut renderer = WinitRenderer {
        windows: HashMap::default(),
        #[cfg(feature = "tray")]
        tray: launch_config.tray,
        #[cfg(all(feature = "tray", not(target_os = "linux")))]
        tray_icon: None,
        resumed: false,
        futures: launch_config
            .tasks
            .into_iter()
            .map(|task| task(LaunchProxy(proxy.clone())))
            .collect::<Vec<_>>(),
        proxy,
        font_manager: font_mgr,
        font_collection,
        windows_configs: launch_config.windows_configs,
        plugins: launch_config.plugins,
        fallback_fonts: launch_config.fallback_fonts,
        screen_reader,
        waker,
        exit_on_close: launch_config.exit_on_close,
    };

    #[cfg(feature = "tray")]
    {
        use crate::{
            renderer::{
                NativeTrayEvent,
                NativeTrayEventAction,
            },
            tray::{
                TrayIconEvent,
                menu::MenuEvent,
            },
        };

        let proxy = renderer.proxy.clone();
        MenuEvent::set_event_handler(Some(move |event| {
            let _ = proxy.send_event(NativeEvent::Tray(NativeTrayEvent {
                action: NativeTrayEventAction::MenuEvent(event),
            }));
        }));
        let proxy = renderer.proxy.clone();
        TrayIconEvent::set_event_handler(Some(move |event| {
            let _ = proxy.send_event(NativeEvent::Tray(NativeTrayEvent {
                action: NativeTrayEventAction::TrayEvent(event),
            }));
        }));

        #[cfg(target_os = "linux")]
        if let Some(tray_icon) = renderer.tray.0.take() {
            std::thread::spawn(move || {
                if !gtk::is_initialized() {
                    if gtk::init().is_ok() {
                        tracing::debug!("Tray: GTK initialized");
                    } else {
                        tracing::error!("Tray: Failed to initialize GTK");
                    }
                }

                let _tray_icon = (tray_icon)();

                gtk::main();
            });
        }
    }

    event_loop.run_app(&mut renderer).unwrap();
}
