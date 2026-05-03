use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    define_theme,
    get_theme,
};

define_theme! {
    %[component]
    pub Slider {
        %[fields]
        background: Color,
        thumb_background: Color,
        thumb_inner_background: Color,
        border_fill: Color,
    }
}

/// Slider component.
///
/// You must pass a percentage from 0.0 to 100.0 and listen for value changes with `on_moved` and then decide if this changes are applicable,
/// and if so, apply them.
///
/// # Example
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut percentage = use_state(|| 25.0);
///
///     Slider::new(move |per| percentage.set(per)).value(percentage())
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().padding(48.).center().expanded().child(app())
/// # }, "./images/gallery_slider.png").render();
/// ```
/// # Preview
/// ![Slider Preview][slider]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("slider", "images/gallery_slider.png")
)]
#[derive(Clone, PartialEq)]
pub struct Slider {
    pub(crate) theme: Option<SliderThemePartial>,
    value: f64,
    on_moved: EventHandler<f64>,
    size: Size,
    direction: Direction,
    enabled: bool,
    cursor_icon: CursorIcon,
    key: DiffKey,
}

impl KeyExt for Slider {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Slider {
    pub fn new(on_moved: impl Into<EventHandler<f64>>) -> Self {
        Self {
            theme: None,
            value: 0.0,
            on_moved: on_moved.into(),
            size: Size::fill(),
            direction: Direction::Horizontal,
            enabled: true,
            cursor_icon: CursorIcon::default(),
            key: DiffKey::None,
        }
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(0.0, 100.0);
        self
    }

    pub fn theme(mut self, theme: SliderThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Override the cursor icon shown when hovering over this component while enabled.
    pub fn cursor_icon(mut self, cursor_icon: impl Into<CursorIcon>) -> Self {
        self.cursor_icon = cursor_icon.into();
        self
    }
}

impl Component for Slider {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, SliderThemePreference, "slider");
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut hovering = use_state(|| false);
        let mut clicking = use_state(|| false);
        let mut size = use_state(Area::default);

        let enabled = use_reactive(&self.enabled);
        let cursor_icon = self.cursor_icon;
        use_drop(move || {
            if hovering() {
                Cursor::set(CursorIcon::default());
            }
        });

        let direction_is_vertical = self.direction == Direction::Vertical;
        let value = self.value;
        let on_moved = self.on_moved.clone();

        let on_key_down = {
            let on_moved = self.on_moved.clone();
            move |e: Event<KeyboardEventData>| match e.key {
                Key::Named(NamedKey::ArrowLeft) if !direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value - 4.0).clamp(0.0, 100.0));
                }
                Key::Named(NamedKey::ArrowRight) if !direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value + 4.0).clamp(0.0, 100.0));
                }
                Key::Named(NamedKey::ArrowUp) if direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value + 4.0).clamp(0.0, 100.0));
                }
                Key::Named(NamedKey::ArrowDown) if direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value - 4.0).clamp(0.0, 100.0));
                }
                _ => {}
            }
        };

        let on_pointer_enter = move |_| {
            hovering.set(true);
            if enabled() {
                Cursor::set(cursor_icon);
            } else {
                Cursor::set(CursorIcon::NotAllowed);
            }
        };

        let on_pointer_leave = move |_| {
            Cursor::set(CursorIcon::default());
            hovering.set(false);
        };

        let calc_percentage = move |x: f64, y: f64| -> f64 {
            let pct = if direction_is_vertical {
                let y = y - 8.0;
                100. - (y / (size.read().height() as f64 - 15.0) * 100.0)
            } else {
                let x = x - 8.0;
                x / (size.read().width() as f64 - 15.) * 100.0
            };
            pct.clamp(0.0, 100.0)
        };

        let on_pointer_down = {
            let on_moved = self.on_moved.clone();
            move |e: Event<PointerEventData>| {
                focus.request_focus();
                clicking.set(true);
                e.stop_propagation();
                let coordinates = e.element_location();
                on_moved.call(calc_percentage(coordinates.x, coordinates.y));
            }
        };

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
            clicking.set(false);
        };

        let on_global_pointer_move = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            if *clicking.peek() {
                let coordinates = e.global_location();
                on_moved.call(calc_percentage(
                    coordinates.x - size.read().min_x() as f64,
                    coordinates.y - size.read().min_y() as f64,
                ));
            }
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme.border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(Color::TRANSPARENT)
                .width(0.)
                .alignment(BorderAlignment::Inner)
        };

        let (slider_width, slider_height) = if direction_is_vertical {
            (Size::px(6.), self.size.clone())
        } else {
            (self.size.clone(), Size::px(6.))
        };

        let track_size = Size::func_data(
            move |ctx| Some(value as f32 / 100. * (ctx.parent - 15.)),
            &(value as i32),
        );

        let (track_width, track_height) = if direction_is_vertical {
            (Size::px(6.), track_size)
        } else {
            (track_size, Size::px(6.))
        };

        let (thumb_offset_x, thumb_offset_y) = if direction_is_vertical {
            (-6., 3.)
        } else {
            (-3., -6.)
        };

        let thumb_main_align = if direction_is_vertical {
            Alignment::end()
        } else {
            Alignment::start()
        };

        let padding = if direction_is_vertical {
            (0., 8.)
        } else {
            (8., 0.)
        };

        let thumb = rect()
            .width(Size::fill())
            .offset_x(thumb_offset_x)
            .offset_y(thumb_offset_y)
            .child(
                rect()
                    .width(Size::px(18.))
                    .height(Size::px(18.))
                    .corner_radius(50.)
                    .background(theme.thumb_background.mul_if(!self.enabled, 0.85))
                    .padding(4.)
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::fill())
                            .background(theme.thumb_inner_background.mul_if(!self.enabled, 0.85))
                            .corner_radius(50.),
                    ),
            );

        let track = rect()
            .width(track_width)
            .height(track_height)
            .background(theme.thumb_inner_background.mul_if(!self.enabled, 0.85))
            .corner_radius(50.);

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .a11y_role(AccessibilityRole::Slider)
            .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
            .maybe(self.enabled, |rect| {
                rect.on_key_down(on_key_down)
                    .on_pointer_down(on_pointer_down)
                    .on_global_pointer_move(on_global_pointer_move)
                    .on_global_pointer_press(on_global_pointer_press)
            })
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .border(border)
            .corner_radius(50.)
            .padding(padding)
            .child(
                rect()
                    .width(slider_width)
                    .height(slider_height)
                    .background(theme.background.mul_if(!self.enabled, 0.85))
                    .corner_radius(50.)
                    .direction(self.direction)
                    .main_align(thumb_main_align)
                    .children(if direction_is_vertical {
                        vec![thumb.into(), track.into()]
                    } else {
                        vec![track.into(), thumb.into()]
                    }),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
