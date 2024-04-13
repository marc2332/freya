//! # Freya Hooks
//! A collection of hooks to be used in Freya.

mod editor_history;
mod rope_editor;
mod shader_uniforms;
mod text_editor;
mod theming;

mod use_accessibility;
mod use_activable_route;
mod use_animation;
mod use_asset_cacher;
mod use_canvas;
mod use_editable;
mod use_focus;
mod use_node;
mod use_platform;
mod use_theme;

#[cfg(feature = "use_camera")]
mod use_camera;

pub use editor_history::*;
pub use rope_editor::*;
pub use shader_uniforms::*;
pub use text_editor::*;
pub use theming::*;

pub use use_accessibility::*;
pub use use_activable_route::*;
pub use use_animation::*;
pub use use_asset_cacher::*;
pub use use_canvas::*;
pub use use_editable::*;
pub use use_focus::*;
pub use use_node::*;
pub use use_platform::*;
pub use use_theme::*;

#[cfg(feature = "use_camera")]
pub use use_camera::*;
