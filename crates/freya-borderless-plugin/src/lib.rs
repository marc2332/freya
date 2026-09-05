use freya_core::prelude::*;
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
    prelude::{
        Area,
        Point2D,
        Size2D,
    },
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
/// - **All**: an optional corner radius rounds the app and a drop shadow is painted around it.
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

    fn on_event(&mut self, _event: &mut PluginEvent, _handle: PluginHandle) {}

    fn root_component(&self, root: Element) -> Element {
        BorderlessRoot {
            thickness: self.thickness,
            corner_radius: self.corner_radius,
            inner: root,
        }
        .into_element()
    }
}

#[derive(Clone, PartialEq)]
struct BorderlessRoot {
    thickness: f32,
    corner_radius: f32,
    inner: Element,
}

impl Component for BorderlessRoot {
    fn render(&self) -> impl IntoElement {
        let maximized = use_maximized();
        let inset = if maximized() { 0. } else { 12. };

        rect()
            .expanded()
            .padding(inset)
            .child(
                rect()
                    .expanded()
                    .overflow(Overflow::Clip)
                    .maybe(!maximized(), |el| {
                        el.corner_radius(self.corner_radius)
                            .shadow((0., 0., inset, 0., Color::BLACK.with_a(90)))
                    })
                    .child(self.inner.clone()),
            )
            .maybe(!cfg!(target_os = "macos"), |el| {
                el.child(ResizeBands::new(self.thickness).with_inset(inset))
            })
    }
}

/// Whether the window is fullscreen or maximized.
pub fn use_maximized() -> State<bool> {
    let mut maximized = use_state(|| false);

    use_side_effect(move || {
        let _ = Platform::get().root_size.read();
        Platform::get().with_window(Platform::window_id(), move |window| {
            if let Some(mut maximized) = maximized.try_write() {
                *maximized = window.fullscreen().is_some() || window.is_maximized();
            }
        });
    });

    maximized
}

/// Invisible bands along the window borders that drive a native resize.
#[derive(PartialEq)]
pub struct ResizeBands {
    thickness: f32,
    inset: f32,
    key: DiffKey,
}

impl KeyExt for ResizeBands {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ResizeBands {
    /// Create the bands, with `thickness` as how far into the window each one reaches.
    pub fn new(thickness: f32) -> Self {
        Self {
            thickness,
            inset: 0.,
            key: DiffKey::None,
        }
    }

    /// Grow the bands inwards by `inset`, so they still reach the app's edges
    /// when it does not span the whole window.
    pub fn with_inset(mut self, inset: f32) -> Self {
        self.inset = inset;
        self
    }
}

impl Component for ResizeBands {
    fn render(&self) -> impl IntoElement {
        let maximized = use_maximized();
        let size = *Platform::get().root_size.read();

        rect()
            .layer(Layer::Overlay)
            .width(Size::px(0.))
            .height(Size::px(0.))
            .maybe(!maximized(), |el| {
                el.children(
                    DIRECTIONS
                        .iter()
                        .map(|direction| self.band(*direction, size)),
                )
            })
    }
}

impl ResizeBands {
    fn band(&self, direction: ResizeDirection, size: Size2D) -> Element {
        let band = self.inset + self.thickness;
        let corner = band + self.thickness;
        let span_x = (size.width - corner * 2.).max(0.);
        let span_y = (size.height - corner * 2.).max(0.);
        let far_x = (size.width - band).max(0.);
        let far_y = (size.height - band).max(0.);
        let corner_far_x = (size.width - corner).max(0.);
        let corner_far_y = (size.height - corner).max(0.);

        let (left, top, width, height) = match direction {
            ResizeDirection::North => (corner, 0., span_x, band),
            ResizeDirection::South => (corner, far_y, span_x, band),
            ResizeDirection::West => (0., corner, band, span_y),
            ResizeDirection::East => (far_x, corner, band, span_y),
            ResizeDirection::NorthWest => (0., 0., corner, corner),
            ResizeDirection::NorthEast => (corner_far_x, 0., corner, corner),
            ResizeDirection::SouthWest => (0., corner_far_y, corner, corner),
            ResizeDirection::SouthEast => (corner_far_x, corner_far_y, corner, corner),
        };
        let area = Area::new(Point2D::new(left, top), Size2D::new(width, height));

        rect()
            .position(
                Position::new_global()
                    .top(area.origin.y)
                    .left(area.origin.x),
            )
            .width(Size::px(area.width()))
            .height(Size::px(area.height()))
            .cursor(cursor(direction))
            .on_pointer_down(move |_| {
                Platform::get().with_window(Platform::window_id(), move |window| {
                    let _ = window.drag_resize_window(direction);
                });
            })
            .into_element()
    }
}

fn cursor(direction: ResizeDirection) -> CursorIcon {
    match direction {
        ResizeDirection::North | ResizeDirection::South => CursorIcon::NsResize,
        ResizeDirection::West | ResizeDirection::East => CursorIcon::EwResize,
        ResizeDirection::NorthWest | ResizeDirection::SouthEast => CursorIcon::NwseResize,
        ResizeDirection::NorthEast | ResizeDirection::SouthWest => CursorIcon::NeswResize,
    }
}
