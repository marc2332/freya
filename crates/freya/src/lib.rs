#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
//! # Freya
//!
//! **Freya** is a declarative, cross-platform GUI Rust library, powered by 🎨 [Skia](https://skia.org/).
//!
//! ### Basics
//! - [Introduction](self::_docs::introduction)
//! - [UI](self::_docs::ui)
//! - [Elements](self::elements)
//! - [Components and Props](self::_docs::components_and_props)
//! - [Hooks](self::_docs::hooks)
//!
//! ### Learn
//! - [Development Setup](self::_docs::development_setup)
//! - [i18n](self::_docs::i18n)
//! - [Built-in Components Gallery](crate::components::gallery)
//!
//! ## Features flags
//!
//! - `all`: Enables all the features listed below
//! - `router`: Reexport `freya-router` under `freya::router`
//! - `i18n`: Reexport `freya-i18n` under `freya::router`
//! - `remote-asset`: Enables support for **HTTP** asset sources for [ImageViewer](freya_components::image_viewer::ImageViewer) and [freya_components::gif_viewer::GifViewer] components.
//! - `devtools`: Enables devtools support.
//! - `performance`: Enables the performance overlay plugin.
//! - `vulkan`: Enables Vulkan rendering support.
//! - `tray`: Enables tray support using the `tray-icon` crate.
//! - `sdk`: Reexport `freya-sdk` under `freya::sdk`
//! - `gif`: Enables the `GifViewer` component.
//! - `plot`: Enables the `plot` element.

pub mod prelude {
    cfg_if::cfg_if! {
        if #[cfg(feature = "router")] {
            pub use freya_components::activable_route::*;
            pub use freya_components::link::*;
            pub use freya_components::native_router::*;
        }
    }
    cfg_if::cfg_if! {
        if #[cfg(feature = "plot")] {
            pub use freya_components::plot::*;
        }
    }
    pub use freya_core::prelude::*;
    pub use freya_winit::{
        WinitEventNotifierExt,
        config::{
            LaunchConfig,
            WindowConfig,
        },
    };

    pub use crate::components::*;
    pub fn launch(launch_config: LaunchConfig) {
        #[cfg(feature = "devtools")]
        let launch_config = launch_config.with_plugin(freya_devtools::DevtoolsPlugin::default());
        #[cfg(feature = "performance")]
        let launch_config = launch_config
            .with_plugin(freya_performance_plugin::PerformanceOverlayPlugin::default());
        freya_winit::launch(launch_config)
    }
    pub use torin::{
        alignment::Alignment,
        content::Content,
        direction::Direction,
        gaps::Gaps,
        geometry::{
            Area,
            CursorPoint,
            Size2D,
        },
        position::Position,
        size::Size,
    };
}
pub mod elements {
    pub use freya_core::elements::*;
}

pub mod components {
    #[cfg(feature = "gif")]
    pub use freya_components::gif_viewer::*;
    pub use freya_components::{
        accordion::*,
        activable_route_context::*,
        button::*,
        checkbox::*,
        chip::*,
        drag_drop::*,
        draggable_canvas::*,
        dropdown::*,
        element_expansions::*,
        floating_tab::*,
        gallery,
        get_theme,
        icons::{
            arrow::*,
            tick::*,
        },
        image_viewer::*,
        input::*,
        loader::*,
        menu::*,
        popup::*,
        portal::*,
        progressbar::*,
        radio_item::*,
        resizable_container::*,
        scrollviews::*,
        selectable_text::*,
        sidebar::*,
        slider::*,
        switch::*,
        table::*,
        theming::{
            component_themes::*,
            hooks::*,
            themes::*,
        },
        tile::*,
        tooltip::*,
    };
}

pub mod text_edit {
    pub use freya_edit::*;
}
pub mod animation {
    pub use freya_animation::prelude::*;
}
#[cfg(feature = "router")]
pub mod router {
    pub use freya_router::*;
}
#[cfg(feature = "i18n")]
pub mod i18n {
    pub use freya_i18n::*;
}
#[cfg(feature = "engine")]
pub mod engine {
    pub use freya_engine::*;
}

pub mod winit {
    pub use freya_winit::winit::*;
}

pub mod helpers {
    pub use freya_core::helpers::*;
}

#[cfg(feature = "tray")]
pub mod tray {
    pub use freya_winit::tray::*;
}

#[cfg(feature = "sdk")]
pub mod sdk {
    pub use freya_sdk::*;
}

#[cfg(doc)]
pub mod _docs;
