#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
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
//! - [Built-in Components](crate::components)
//! - [Built-in Components Gallery](crate::components::gallery)
//!
//! ## Features flags
//!
//! - `all`: Enables all the features listed below
//! - `router`: Reexport `freya-router` under [router]
//! - `i18n`: Reexport `freya-i18n` under [i18n]
//! - `remote-asset`: Enables support for **HTTP** asset sources for [ImageViewer](components::ImageViewer) and [GifViewer](components::GifViewer) components.
//! - `devtools`: Enables devtools support.
//! - `performance`: Enables the performance overlay plugin.
//! - `vulkan`: Enables Vulkan rendering support.
//! - `tray`: Enables tray support using the [tray_icon] crate.
//! - `sdk`: Reexport `freya-sdk` under [sdk]
//! - `gif`: Enables the [GifViewer](components::GifViewer) component.
//! - `plot`: Enables the [plot](prelude::plot) element.
//! - `material-design`: Reexport `freya-material-design` under [material_design].
//! - `calendar`: Enables the [Calendar](components::Calendar) component.
//! - `hotpath`: Enables Freya's internal tracking using hotpath.

pub mod prelude {
    pub use freya_core::prelude::*;
    pub use freya_winit::{
        WinitPlatformExt,
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
    #[cfg_attr(feature = "docs", doc(cfg(feature = "gif")))]
    #[cfg(feature = "gif")]
    pub use freya_components::gif_viewer::*;
    cfg_if::cfg_if! {
        if #[cfg(feature = "router")] {
            #[cfg_attr(feature = "docs", doc(cfg(feature = "router")))]
            pub use freya_components::activable_route::*;
            pub use freya_components::link::*;
            pub use freya_components::native_router::*;
            pub use freya_components::animated_router::*;
        }
    }
    #[cfg_attr(feature = "docs", doc(cfg(feature = "remote-asset")))]
    #[cfg(feature = "remote-asset")]
    pub use freya_components::Uri;
    #[cfg_attr(feature = "docs", doc(cfg(feature = "calendar")))]
    #[cfg(feature = "calendar")]
    pub use freya_components::calendar::*;
    #[cfg_attr(feature = "docs", doc(cfg(feature = "plot")))]
    #[cfg(feature = "plot")]
    pub use freya_components::plot::*;
    pub use freya_components::{
        accordion::*,
        activable_route_context::*,
        button::*,
        checkbox::*,
        chip::*,
        context_menu::*,
        cursor_area::*,
        drag_drop::*,
        draggable_canvas::*,
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
        overflowed_content::*,
        popup::*,
        portal::*,
        progressbar::*,
        radio_item::*,
        resizable_container::*,
        scrollviews::*,
        segmented_button::*,
        select::*,
        selectable_text::*,
        sidebar::*,
        slider::*,
        switch::*,
        table::*,
        theming::{
            component_themes::*,
            extensions::*,
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

#[cfg_attr(feature = "docs", doc(cfg(feature = "router")))]
#[cfg(feature = "router")]
pub mod router {
    pub use freya_router::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "i18n")))]
#[cfg(feature = "i18n")]
pub mod i18n {
    pub use freya_i18n::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "engine")))]
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

#[cfg_attr(feature = "docs", doc(cfg(feature = "tray")))]
#[cfg(feature = "tray")]
pub mod tray {
    pub use freya_winit::tray::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "sdk")))]
#[cfg(feature = "sdk")]
pub mod sdk {
    pub use freya_sdk::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "material-design")))]
#[cfg(feature = "material-design")]
pub mod material_design {
    pub use freya_material_design::prelude::*;
}

#[cfg(doc)]
pub mod _docs;
