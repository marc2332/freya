use freya_core::prelude::*;
use freya_engine::prelude::{
    ClipOp,
    Color,
    RRect,
    SkRect,
};
use freya_winit::{
    extensions::WinitPlatformExt,
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
    winit::window::ResizeDirection,
};
use torin::{
    position::Position,
    prelude::Size2D,
    size::Size,
};

const DIRECTIONS: [ResizeDirection; 8] = [
    ResizeDirection::North,
    ResizeDirection::South,
    ResizeDirection::West,
    ResizeDirection::East,
    ResizeDirection::NorthWest,
    ResizeDirection::NorthEast,
    ResizeDirection::SouthWest,
    ResizeDirection::SouthEast,
];

/// Plugin for windows without native decorations.
///
/// # Platforms
///
/// - **Linux** and **Windows**: overlays [ResizeBands] on top of the app, so dragging
///   the window borders resizes it and hovering them shows the resize cursors.
/// - **macOS**: no bands, resizing is left to the system.
/// - **All**: an optional corner radius clips the whole canvas, overlay layers included.
///
/// # Example
///
/// ```rust,no_run
/// # use freya_winit::{config::{LaunchConfig, WindowConfig}, launch};
/// # use freya_borderless_plugin::BorderlessPlugin;
/// # fn app() -> impl freya_core::prelude::IntoElement { freya_core::prelude::rect() }
/// launch(
///     LaunchConfig::new()
///         .with_plugin(BorderlessPlugin::new().with_corner_radius(12.))
///         .with_window(WindowConfig::new(app).with_decorations(false)),
/// )
/// ```
pub struct BorderlessPlugin {
    thickness: f32,
    corner_radius: f32,
}

impl Default for BorderlessPlugin {
    fn default() -> Self {
        Self {
            thickness: 6.,
            corner_radius: 0.,
        }
    }
}

impl BorderlessPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    /// How far into the window each resize band reaches.
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Clip the whole canvas with this corner radius, in logical pixels.
    pub fn with_corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl FreyaPlugin for BorderlessPlugin {
    fn plugin_id(&self) -> &'static str {
        "borderless"
    }

    fn on_event(&mut self, event: &mut PluginEvent, _handle: PluginHandle) {
        if self.corner_radius <= 0. {
            return;
        }
        match event {
            PluginEvent::BeforeRender { canvas, window, .. } => {
                canvas.clear(Color::TRANSPARENT);
                canvas.save();
                let size = window.inner_size();
                let radius = self.corner_radius * window.scale_factor() as f32;
                let rounded_window = RRect::new_rect_xy(
                    SkRect::from_wh(size.width as f32, size.height as f32),
                    radius,
                    radius,
                );
                canvas.clip_rrect(rounded_window, ClipOp::Intersect, true);
            }
            PluginEvent::AfterRender { canvas, .. } => {
                canvas.restore();
            }
            _ => {}
        }
    }

    fn root_component(&self, root: Element) -> Element {
        if cfg!(target_os = "macos") {
            return root;
        }
        BorderlessRoot {
            thickness: self.thickness,
            inner: root,
        }
        .into_element()
    }
}

#[derive(Clone, PartialEq)]
struct BorderlessRoot {
    thickness: f32,
    inner: Element,
}

impl Component for BorderlessRoot {
    fn render(&self) -> impl IntoElement {
        rect()
            .expanded()
            .child(self.inner.clone())
            .child(ResizeBands {
                thickness: self.thickness,
            })
    }
}

/// Whether the window is fullscreen or maximized.
pub fn use_edge_to_edge() -> State<bool> {
    let mut edge_to_edge = use_state(|| false);

    use_side_effect(move || {
        let _ = Platform::get().root_size.read();
        Platform::get().with_window(None, move |window| {
            if let Some(mut edge_to_edge) = edge_to_edge.try_write() {
                *edge_to_edge = window.fullscreen().is_some() || window.is_maximized();
            }
        });
    });

    edge_to_edge
}

/// Invisible bands along the window borders that drive a native resize.
#[derive(PartialEq)]
pub struct ResizeBands {
    /// How far into the window each band reaches.
    pub thickness: f32,
}

impl Component for ResizeBands {
    fn render(&self) -> impl IntoElement {
        let edge_to_edge = use_edge_to_edge();

        if edge_to_edge() {
            return rect().into_element();
        }

        let size = *Platform::get().root_size.read();
        let thickness = self.thickness;

        rect()
            .layer(Layer::Overlay)
            .width(Size::px(0.))
            .height(Size::px(0.))
            .children(
                DIRECTIONS
                    .iter()
                    .map(|direction| band(*direction, size, thickness)),
            )
            .into_element()
    }
}

fn band(direction: ResizeDirection, size: Size2D, thickness: f32) -> Element {
    let (left, top, width, height) = geometry(direction, size, thickness);

    rect()
        .position(Position::new_global().top(top).left(left))
        .width(Size::px(width))
        .height(Size::px(height))
        .on_pointer_enter(move |_| Cursor::set(cursor(direction)))
        .on_pointer_leave(move |_| Cursor::set(CursorIcon::Default))
        .on_pointer_down(move |_| {
            Platform::get().with_window(None, move |window| {
                let _ = window.drag_resize_window(direction);
            });
        })
        .into_element()
}

/// Band placement as `(left, top, width, height)`.
fn geometry(direction: ResizeDirection, size: Size2D, thickness: f32) -> (f32, f32, f32, f32) {
    let corner = thickness * 2.;
    let span_x = (size.width - corner).max(0.);
    let span_y = (size.height - corner).max(0.);
    let far_x = (size.width - thickness).max(0.);
    let far_y = (size.height - thickness).max(0.);

    match direction {
        ResizeDirection::North => (thickness, 0., span_x, thickness),
        ResizeDirection::South => (thickness, far_y, span_x, thickness),
        ResizeDirection::West => (0., thickness, thickness, span_y),
        ResizeDirection::East => (far_x, thickness, thickness, span_y),
        ResizeDirection::NorthWest => (0., 0., corner, corner),
        ResizeDirection::NorthEast => (span_x, 0., corner, corner),
        ResizeDirection::SouthWest => (0., span_y, corner, corner),
        ResizeDirection::SouthEast => (span_x, span_y, corner, corner),
    }
}

fn cursor(direction: ResizeDirection) -> CursorIcon {
    match direction {
        ResizeDirection::North => CursorIcon::NResize,
        ResizeDirection::South => CursorIcon::SResize,
        ResizeDirection::West => CursorIcon::WResize,
        ResizeDirection::East => CursorIcon::EResize,
        ResizeDirection::NorthWest => CursorIcon::NwResize,
        ResizeDirection::NorthEast => CursorIcon::NeResize,
        ResizeDirection::SouthWest => CursorIcon::SwResize,
        ResizeDirection::SouthEast => CursorIcon::SeResize,
    }
}
