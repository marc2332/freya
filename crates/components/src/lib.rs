//! # Freya Components
//! A collection of basic components to be used in Freya.
//!
//! Go to [Gallery](gallery) to see previews of the components.

/// | 1 | 2 | 3 | 4 |
/// | ------------- | ------------- | ------------- | ------------- |
/// | ![Switch Preview][gallery_enabled_switch] | ![Button Preview][gallery_button] | ![Slider Preview][gallery_slider] | ![Checkbox Preview][gallery_checkbox] |
/// | [Switch]        | [Button]        | [Slider]        | [Checkbox] |
/// | ![Radio Preview][gallery_radio] |  ![Input Preview][gallery_input] | ![ProgressBar Preview][gallery_progress_bar] |
/// | [Radio]        | [Input]        | [ProgressBar]        |
#[embed_doc_image::embed_doc_image("gallery_button", "images/gallery_button.png")]
#[embed_doc_image::embed_doc_image("gallery_enabled_switch", "images/gallery_enabled_switch.png")]
#[embed_doc_image::embed_doc_image("gallery_slider", "images/gallery_slider.png")]
#[embed_doc_image::embed_doc_image("gallery_checkbox", "images/gallery_checkbox.png")]
#[embed_doc_image::embed_doc_image("gallery_radio", "images/gallery_radio.png")]
#[embed_doc_image::embed_doc_image("gallery_input", "images/gallery_input.png")]
#[embed_doc_image::embed_doc_image("gallery_progress_bar", "images/gallery_progress_bar.png")]
pub fn gallery() {}

mod accordion;
mod activable_route;
mod animated_router;
mod body;
mod button;
mod canvas;
mod checkbox;
mod cursor_area;
mod drag_drop;
mod dropdown;
mod gesture_area;
mod graph;
mod hooks;
mod icons;
mod image;
mod input;
mod link;
mod loader;
mod menu;
mod native_container;
mod native_router;
mod network_image;
mod popup;
mod progress_bar;
mod radio;
mod scroll_views;
mod sidebar;
mod slider;
mod snackbar;
mod svg;
mod switch;
mod table;
mod tabs;
mod theme;
mod tile;
mod tooltip;
mod tree;
mod window_drag_area;

pub use accordion::*;
pub use activable_route::*;
pub use animated_router::*;
pub use body::*;
pub use button::*;
pub use canvas::*;
pub use checkbox::*;
pub use cursor_area::*;
pub use drag_drop::*;
pub use dropdown::*;
pub use gesture_area::*;
pub use graph::*;
pub use hooks::*;
pub use icons::*;
pub use input::*;
pub use link::*;
pub use loader::*;
pub use menu::*;
pub use native_container::*;
pub use native_router::*;
pub use network_image::*;
pub use popup::*;
pub use progress_bar::*;
pub use radio::*;
pub use scroll_views::*;
pub use sidebar::*;
pub use slider::*;
pub use snackbar::*;
pub use switch::*;
pub use table::*;
pub use tabs::*;
pub use theme::*;
pub use tile::*;
pub use tooltip::*;
pub use tree::*;
pub use window_drag_area::*;
