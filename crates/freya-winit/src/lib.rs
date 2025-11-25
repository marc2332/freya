pub mod reexports {
    pub use winit;
}

use crate::{
    config::LaunchConfig,
    renderer::{
        NativeEvent,
        WinitRenderer,
    },
};
mod accessibility;
pub mod config;
mod drivers;
pub mod extensions;
pub mod plugins;
pub mod renderer;
mod window;
mod winit_mappings;

pub use extensions::*;

pub mod winit {
    pub use winit::*;
}

pub fn launch(launch_config: LaunchConfig) {
    use std::collections::HashMap;

    use freya_core::integration::*;
    use freya_engine::prelude::{
        FontCollection,
        FontMgr,
        TypefaceFontProvider,
    };
    use winit::event_loop::EventLoop;

    let mut event_loop_builder = EventLoop::<NativeEvent>::with_user_event();

    let event_loop = event_loop_builder
        .build()
        .expect("Failed to create event loop.");

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

    let mut renderer = WinitRenderer {
        windows: HashMap::default(),
        resumed: false,
        proxy,
        font_manager: font_mgr,
        font_collection,
        windows_configs: launch_config.windows_configs,
        plugins: launch_config.plugins,
        fallback_fonts: launch_config.fallback_fonts,
        screen_reader,
    };

    event_loop.run_app(&mut renderer).unwrap();
}
