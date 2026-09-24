#![cfg_attr(feature = "docs", feature(doc_cfg))]

pub mod accordion;
pub mod activable;
pub mod activable_context;
pub mod attached;
pub mod button;
pub mod cache;
#[cfg(feature = "calendar")]
pub mod calendar;
pub mod canvas;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod color_picker;
pub mod context_menu;
pub mod cursor_blink;
pub mod docking;
pub mod drag_drop;
pub mod draggable_canvas;
pub mod element_expansions;
pub mod floating_tab;
#[cfg(feature = "remote-asset")]
pub mod http;
pub mod icons;
pub mod image_viewer;
pub mod input;
pub mod integration;
pub mod lazy;
pub mod loader;
pub mod menu;
pub mod overflowed_content;
pub mod popup;
pub mod portal;
pub mod progressbar;
pub mod radio_item;
pub mod resizable_container;
pub mod scrollviews;
pub mod segmented_button;
pub mod select;
pub mod selectable_text;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod svg_viewer;
pub mod switch;
pub mod table;
pub mod theming;
pub mod tile;
#[cfg(feature = "titlebar")]
pub mod titlebar;
pub mod tooltip;
pub mod typography;

#[cfg(feature = "remote-asset")]
pub use url::Url;

cfg_if::cfg_if! {
    if #[cfg(feature = "router")] {
        pub mod activable_route;
        pub mod link;
        pub mod native_router;
        pub mod animated_router;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "gif")] {
        pub mod gif_viewer;
    }
}

/// See the [interactive components demo](https://freyaui.dev/demo).
pub fn gallery() {}
