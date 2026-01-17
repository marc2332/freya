#![cfg_attr(feature = "docs", feature(doc_cfg))]

pub mod accordion;
pub mod activable_route_context;
pub mod button;
pub mod cache;
#[cfg(feature = "calendar")]
pub mod calendar;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod color_picker;
pub mod context_menu;
pub mod cursor_area;
pub mod cursor_blink;
pub mod drag_drop;
pub mod draggable_canvas;
pub mod element_expansions;
pub mod floating_tab;
pub mod icons;
pub mod image_viewer;
pub mod input;
pub mod integration;
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
pub mod slider;
pub mod switch;
pub mod table;
pub mod theming;
pub mod tile;
pub mod tooltip;

#[cfg(feature = "remote-asset")]
pub use ureq::http::Uri;

cfg_if::cfg_if! {
    if #[cfg(feature = "router")] {
        pub mod activable_route;
        pub mod link;
        pub mod native_router;
        pub mod animated_router;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "plot")] {
        pub mod plot;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "gif")] {
        pub mod gif_viewer;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "markdown")] {
        pub mod markdown;
    }
}

/// | 1 | 2 | 3 | 4 |
/// | ------------- | ------------- | ------------- | ------------- |
/// | ![Switch Preview][gallery_toggled_switch] | ![Button Preview][gallery_button] | ![Filled Button Preview][gallery_filled_button] | ![Outline Button Preview][gallery_outline_button] |
/// | [Switch](switch::Switch)        | [Button](button::Button)        | [Filled Button](button::Button)        | [Outline Button](button::Button)        |
/// | ![Flat Button Preview][gallery_flat_button] | ![Slider Preview][gallery_slider] | ![Checkbox Preview][gallery_checkbox] | ![RadioItem Preview][gallery_radio] |
/// | [Flat Button](button::Button)        | [Slider](slider::Slider)        | [Checkbox](checkbox::Checkbox)        | [RadioItem](radio_item::RadioItem)        |
/// | ![ProgressBar Preview][gallery_progressbar] | ![Select Preview][gallery_select] | ![GifViewer Preview][gallery_gif_viewer] | ![Accordion Preview][gallery_accordion] |
/// | [ProgressBar](progressbar::ProgressBar)        | [Select](select::Select)        | [GifViewer](gif_viewer::GifViewer) | [Accordion](accordion::Accordion) |
/// | ![Floating Tab Preview][gallery_floating_tab] | ![ImageViewer Preview][gallery_image_viewer] | ![ScrollView Preview][gallery_scrollview] | ![VirtualScrollView Preview][gallery_virtual_scrollview] |
/// | [FloatingTab](floating_tab::FloatingTab)        | [ImageViewer](image_viewer::ImageViewer) | [ScrollView](scrollviews::ScrollView) | [VirtualScrollView](scrollviews::VirtualScrollView) |
/// | ![Circular Loader Preview][gallery_circular_loader] | ![SegmentedButton Preview][gallery_segmented_button] | ![Tooltip Preview][gallery_tooltip] | ![Calendar Preview][gallery_calendar] |
/// | [CircularLoader](loader::CircularLoader)        | [SegmentedButton](segmented_button::SegmentedButton) | [Tooltip](tooltip::Tooltip) | [Calendar](calendar::Calendar) |
/// | ![ColorPicker Preview][gallery_color_picker] | ![Chip Preview][gallery_chip] | ![Menu Preview][gallery_menu] | |
/// | [ColorPicker](color_picker::ColorPicker)        | [Chip](chip::Chip)        | [Menu](menu::Menu)        |      |
/// | ![Popup Preview][gallery_popup] | ![Portal Preview][gallery_portal] | ![ResizableContainer Preview][gallery_resizable_container] | ![Sidebar Preview][gallery_sidebar] |
/// | [Popup](popup::Popup)        | [Portal](portal::Portal)        | [ResizableContainer](resizable_container::ResizableContainer)        | [Sidebar](sidebar::SideBar)        |
/// | ![Table Preview][gallery_table] | ![Tile Preview][gallery_tile] | | |
/// | [Table](table::Table)        | [Tile](tile::Tile)        |        |        |
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("gallery_button", "images/gallery_button.png"),
    doc = embed_doc_image::embed_image!("gallery_filled_button", "images/gallery_filled_button.png"),
    doc = embed_doc_image::embed_image!("gallery_outline_button", "images/gallery_outline_button.png"),
    doc = embed_doc_image::embed_image!("gallery_toggled_switch", "images/gallery_toggled_switch.png"),
    doc = embed_doc_image::embed_image!("gallery_slider", "images/gallery_slider.png"),
    doc = embed_doc_image::embed_image!("gallery_checkbox", "images/gallery_checkbox.png"),
    doc = embed_doc_image::embed_image!("gallery_radio", "images/gallery_radio.png"),
    doc = embed_doc_image::embed_image!("gallery_input", "images/gallery_input.png"),
    doc = embed_doc_image::embed_image!("gallery_progressbar", "images/gallery_progressbar.png"),
    doc = embed_doc_image::embed_image!("gallery_select", "images/gallery_select.png"),
    doc = embed_doc_image::embed_image!("gallery_accordion", "images/gallery_accordion.png"),
    doc = embed_doc_image::embed_image!("gallery_floating_tab", "images/gallery_floating_tab.png"),
    doc = embed_doc_image::embed_image!("gallery_image_viewer", "images/gallery_image_viewer.png"),
    doc = embed_doc_image::embed_image!("gallery_scrollview", "images/gallery_scrollview.png"),
    doc = embed_doc_image::embed_image!("gallery_virtual_scrollview", "images/gallery_virtual_scrollview.png"),
    doc = embed_doc_image::embed_image!("gallery_circular_loader", "images/gallery_circular_loader.png"),
    doc = embed_doc_image::embed_image!("gallery_tooltip", "images/gallery_tooltip.png"),
    doc = embed_doc_image::embed_image!("gallery_gif_viewer", "images/gallery_gif_viewer.png"),
    doc = embed_doc_image::embed_image!("gallery_segmented_button", "images/gallery_segmented_button.png"),
    doc = embed_doc_image::embed_image!("gallery_flat_button", "images/gallery_flat_button.png"),
    doc = embed_doc_image::embed_image!("gallery_calendar", "images/gallery_calendar.png"),
    doc = embed_doc_image::embed_image!("gallery_color_picker", "images/gallery_color_picker.png"),
    doc = embed_doc_image::embed_image!("gallery_chip", "images/gallery_chip.png"),
    doc = embed_doc_image::embed_image!("gallery_menu", "images/gallery_menu.png"),
    doc = embed_doc_image::embed_image!("gallery_popup", "images/gallery_popup.png"),
    doc = embed_doc_image::embed_image!("gallery_resizable_container", "images/gallery_resizable_container.png"),
    doc = embed_doc_image::embed_image!("gallery_sidebar", "images/gallery_sidebar.png"),
    doc = embed_doc_image::embed_image!("gallery_table", "images/gallery_table.png"),
)]
pub fn gallery() {}
