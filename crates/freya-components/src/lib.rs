pub mod accordion;
pub mod activable_route_context;
pub mod button;
pub mod cache;
pub mod checkbox;
pub mod chip;
pub mod drag_drop;
pub mod draggable_canvas;
pub mod dropdown;
pub mod element_expansions;
pub mod floating_tab;
pub mod icons;
pub mod image_viewer;
pub mod input;
pub mod keyboard_navigator;
pub mod loader;
pub mod popup;
pub mod portal;
pub mod progressbar;
pub mod radio_item;
pub mod resizable_container;
pub mod scrollviews;
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

/// | 1 | 2 | 3 | 4 |
/// | ------------- | ------------- | ------------- | ------------- |
/// | ![Switch Preview][gallery_toggled_switch] | ![Button Preview][gallery_button] | ![Filled Button Preview][gallery_filled_button] | ![Outline Button Preview][gallery_outline_button] |
/// | [Switch](switch::Switch)        | [Button](button::Button)        | [Filled Button](button::Button)        | [Outline Button](button::Button)        |
/// | ![Slider Preview][gallery_slider] | ![Checkbox Preview][gallery_checkbox] | ![RadioItem Preview][gallery_radio] | ![Input Preview][gallery_input] |
/// | [Slider](slider::Slider)        | [Checkbox](checkbox::Checkbox)        | [RadioItem](radio_item::RadioItem)        | [Input](input::Input)        |
/// | ![ProgressBar Preview][gallery_progressbar] | ![Dropdown Preview][gallery_dropdown] | ![GifViewer Preview][gallery_gif_viewer] | ![Accordion Preview][gallery_accordion] |
/// | [ProgressBar](progressbar::ProgressBar)        | [Dropdown](dropdown::Dropdown)        |  [GifViewer](gif_viewer::GifViewer) | [Accordion](accordion::Accordion) |
/// | ![Floating Tab Preview][gallery_floating_tab] | ![ImageViewer Preview][gallery_image_viewer] |  ![ScrollView Preview][gallery_scrollview] |  ![VirtualScrollView Preview][gallery_virtual_scrollview] |
/// | [FloatingTab](floating_tab::FloatingTab)        | [ImageViewer](image_viewer::ImageViewer) | [ScrollView](scrollviews::ScrollView) | [VirtualScrollView](scrollviews::VirtualScrollView) |
/// | ![Circular Loader Preview][gallery_circular_loader] |  | ![Tooltip Preview][gallery_tooltip] |  |
/// | [CircularLoader](loader::CircularLoader)        | | [Tooltip](tooltip::Tooltip) | |
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
    doc = embed_doc_image::embed_image!("gallery_dropdown", "images/gallery_dropdown.png"),
    doc = embed_doc_image::embed_image!("gallery_accordion", "images/gallery_accordion.png"),
    doc = embed_doc_image::embed_image!("gallery_floating_tab", "images/gallery_floating_tab.png"),
    doc = embed_doc_image::embed_image!("gallery_image_viewer", "images/gallery_image_viewer.png"),
    doc = embed_doc_image::embed_image!("gallery_scrollview", "images/gallery_scrollview.png"),
    doc = embed_doc_image::embed_image!("gallery_virtual_scrollview", "images/gallery_virtual_scrollview.png"),
    doc = embed_doc_image::embed_image!("gallery_circular_loader", "images/gallery_circular_loader.png"),
    doc = embed_doc_image::embed_image!("gallery_tooltip", "images/gallery_tooltip.png"),
    doc = embed_doc_image::embed_image!("gallery_gif_viewer", "images/gallery_gif_viewer.png"),
)]
pub fn gallery() {}
