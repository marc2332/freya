#![doc(
    html_logo_url = "https://freyaui.dev/logo.svg",
    html_favicon_url = "https://freyaui.dev/logo.svg"
)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
//! # Freya
//!
//! **Freya** is a declarative, cross-platform GUI 🦀 Rust library, powered by 🎨 [Skia](https://skia.org/).
//!
//! #### Example
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn main() {
//!     // *Start* your app with a window and its root component
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! fn app() -> impl IntoElement {
//!     // Define a reactive *state*
//!     let mut count = use_state(|| 0);
//!
//!     // Declare the *UI*
//!     rect()
//!         .width(Size::fill())
//!         .height(Size::fill())
//!         .background((35, 35, 35))
//!         .color(Color::WHITE)
//!         .padding(Gaps::new_all(12.))
//!         .on_mouse_up(move |_| *count.write() += 1)
//!         .child(format!("Click to increase -> {}", count.read()))
//! }
//! ```
//!
//! ### Basics
//! - [UI and Components](self::_docs::ui_and_components)
//! - [Elements](self::elements)
//! - [Hooks](self::_docs::hooks)
//! - [State](self::_docs::state_management)
//! - [Development Setup](self::_docs::development_setup)
//!
//! ### Learn
//! - [Built-in Components](crate::components)
//! - [Built-in Components Gallery](crate::components::gallery)
//! - [i18n](freya_i18n)
//! - [Animation](freya_animation::prelude::use_animation)
//! - [Routing](freya_router)
//! - [Clipboard](freya_clipboard)
//! - [Icons](freya_icons)
//! - [Material Design](freya_material_design)
//! - [Plotters](freya_plotters_backend)
//! - [Testing](freya_testing)
//! - [WebView](freya_webview)
//! - [Terminal](freya_terminal)
//!
//! ## Features flags
//!
//! - `all`: Enables all the features listed below
//! - `router`: Reexport [freya_router] under [router]
//! - `i18n`: Reexport [freya_i18n] under [i18n]
//! - `remote-asset`: Enables support for **HTTP** asset sources for [ImageViewer](components::ImageViewer) and [GifViewer](components::GifViewer) components.
//! - `tray`: Enables tray support using the [tray_icon] crate.
//! - `sdk`: Reexport [freya_sdk] under [sdk].
//! - `gif`: Enables the [GifViewer](components::GifViewer) component.
//! - `plot`: Reexport of plotters under [plot].
//! - `material-design`: Reexport [freya_material_design] under [material_design].
//! - `calendar`: Enables the [Calendar](components::Calendar) component.
//! - `icons`: Reexport of [freya_icons] under [icons].
//! - `radio`: Reexport [freya_radio] under [radio].
//! - `query`: Reexport [freya_query] under [query].
//! - `markdown`: Enables the [MarkdownViewer](components::MarkdownViewer) component.
//! - `webview`: Reexport [freya_webview] under [webview].
//! - `titlebar`: Enables the [TitlebarButton](components::TitlebarButton) component.
//! - `terminal`: Reexport [freya_terminal] under [terminal].
//!
//! ## Misc features
//! - `devtools`: Enables devtools support.
//! - `performance`: Enables the performance overlay plugin.
//! - `vulkan`: Enables Vulkan rendering support.
//! - `hotpath`: Enables Freya's internal usage of hotpath.

pub mod prelude {
    pub use freya_core::prelude::*;
    pub use freya_edit::{
        Clipboard,
        ClipboardError,
    };
    pub use freya_winit::{
        WindowDragExt,
        WinitPlatformExt,
        config::{
            CloseDecision,
            LaunchConfig,
            WindowConfig,
        },
        renderer::RendererContext,
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

    #[cfg_attr(feature = "docs", doc(cfg(feature = "router")))]
    #[cfg(feature = "router")]
    pub use freya_router;
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
        visible_size::VisibleSize,
    };
}
pub mod elements {
    pub use freya_core::elements::*;
}

pub mod components {
    #[cfg_attr(feature = "docs", doc(cfg(feature = "gif")))]
    #[cfg(feature = "gif")]
    pub use freya_components::gif_viewer::*;
    #[cfg_attr(feature = "docs", doc(cfg(feature = "markdown")))]
    #[cfg(feature = "markdown")]
    pub use freya_components::markdown::*;
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
    #[cfg(feature = "titlebar")]
    pub use freya_components::titlebar::*;
    pub use freya_components::{
        accordion::*,
        activable_route_context::*,
        button::*,
        canvas::*,
        card::*,
        checkbox::*,
        chip::*,
        color_picker::*,
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

pub mod clipboard {
    pub use freya_clipboard::prelude::*;
}

pub mod animation {
    pub use freya_animation::prelude::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "plot")))]
#[cfg(feature = "plot")]
pub mod plot {
    pub use freya_plotters_backend::*;
    pub use plotters;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "router")))]
#[cfg(feature = "router")]
pub mod router {
    pub use freya_router::prelude::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "i18n")))]
#[cfg(feature = "i18n")]
pub mod i18n {
    pub use freya_i18n::prelude::*;
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
    pub use freya_sdk::prelude::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "material-design")))]
#[cfg(feature = "material-design")]
pub mod material_design {
    pub use freya_material_design::prelude::*;
}

#[cfg_attr(feature = "docs", doc(cfg(feature = "icons")))]
#[cfg(feature = "icons")]
pub mod icons {
    pub use freya_icons::*;
}

/// Reexport `freya-radio` when the `radio` feature is enabled.
#[cfg(feature = "radio")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "radio")))]
pub mod radio {
    pub use freya_radio::prelude::*;
}

/// Reexport `freya-query` when the `query` feature is enabled.
#[cfg(feature = "query")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "query")))]
pub mod query {
    pub use freya_query::prelude::*;
}

/// Reexport `freya-webview` when the `webview` feature is enabled.
#[cfg(feature = "webview")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "webview")))]
pub mod webview {
    pub use freya_webview::prelude::*;
}

/// Reexport `freya-terminal` when the `terminal` feature is enabled.
#[cfg(feature = "terminal")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "terminal")))]
pub mod terminal {
    pub use freya_terminal::prelude::*;
}

#[cfg(doc)]
pub mod _docs;
