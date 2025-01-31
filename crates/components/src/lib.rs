//! # Freya Components
//! A collection of basic components to be used in Freya.
//!
//! Go to [Gallery](gallery) to see previews of the components.

/// | 1 | 2 | 3 | 4 |
/// | ------------- | ------------- | ------------- | ------------- |
/// | ![Switch Preview][gallery_enabled_switch] | ![Button Preview][gallery_button] | ![FilledButton Preview][gallery_filled_button] | ![OutlineButton Preview][gallery_outline_button] |
/// | [Switch]        | [Button]        | [FilledButton]        | [OutlineButton]        |
/// | ![Slider Preview][gallery_slider] | ![Checkbox Preview][gallery_checkbox] | ![Radio Preview][gallery_radio] | ![Input Preview][gallery_input] |
/// | [Slider]        | [Checkbox]        | [Radio]        | [Input]        |
/// | ![ProgressBar Preview][gallery_progress_bar] | ![Dropdown Preview][gallery_dropdown] | ![SnackBar Preview][gallery_snackbar] | ![Tab Preview][gallery_tab] |
/// | [ProgressBar]        | [Dropdown]        | [SnackBar]        | [Tab]        |
/// | ![BottomTab Preview][gallery_bottom_tab] | ![NetworkImage Preview][gallery_network_image] |  ![ScrollView Preview][gallery_scroll_view] |  ![VirtualScrollView Preview][gallery_virtual_scroll_view] |
/// | [BottomTab]        | [NetworkImage] | [ScrollView] | [VirtualScrollView] |
/// | ![Loader Preview][gallery_loader] |  ![Menu Preview][gallery_menu] |  |  |
/// | [Loader]        | [Menu] |  |  |
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("gallery_button", "images/gallery_button.png"),
    doc = embed_doc_image::embed_image!("gallery_filled_button", "images/gallery_filled_button.png"),
    doc = embed_doc_image::embed_image!("gallery_outline_button", "images/gallery_outline_button.png"),
    doc = embed_doc_image::embed_image!("gallery_enabled_switch", "images/gallery_enabled_switch.png"),
    doc = embed_doc_image::embed_image!("gallery_slider", "images/gallery_slider.png"),
    doc = embed_doc_image::embed_image!("gallery_checkbox", "images/gallery_checkbox.png"),
    doc = embed_doc_image::embed_image!("gallery_radio", "images/gallery_radio.png"),
    doc = embed_doc_image::embed_image!("gallery_input", "images/gallery_input.png"),
    doc = embed_doc_image::embed_image!("gallery_progress_bar", "images/gallery_progress_bar.png"),
    doc = embed_doc_image::embed_image!("gallery_dropdown", "images/gallery_dropdown.png"),
    doc = embed_doc_image::embed_image!("gallery_snackbar", "images/gallery_snackbar.png"),
    doc = embed_doc_image::embed_image!("gallery_tab", "images/gallery_tab.png"),
    doc = embed_doc_image::embed_image!("gallery_bottom_tab", "images/gallery_bottom_tab.png"),
    doc = embed_doc_image::embed_image!("gallery_network_image", "images/gallery_network_image.png"),
    doc = embed_doc_image::embed_image!("gallery_scroll_view", "images/gallery_scroll_view.png"),
    doc = embed_doc_image::embed_image!("gallery_virtual_scroll_view", "images/gallery_virtual_scroll_view.png"),
    doc = embed_doc_image::embed_image!("gallery_loader", "images/gallery_loader.png"),
    doc = embed_doc_image::embed_image!("gallery_menu", "images/gallery_menu.png"),
)]
pub fn gallery() {}

mod accordion;
mod activable_route;
mod animated_position;
mod animated_router;
mod body;
mod button;
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
mod overflowed_content;
mod popup;
mod progress_bar;
mod radio;
mod resizable_container;
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
pub use animated_position::*;
pub use animated_router::*;
pub use body::*;
pub use button::*;
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
pub use overflowed_content::*;
pub use popup::*;
pub use progress_bar::*;
pub use radio::*;
pub use resizable_container::*;
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
