use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    get_theme,
    theming::component_themes::SliderThemePartial,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SliderStatus {
    #[default]
    Idle,
    Hovering,
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
///     Slider::new(move |per| percentage.set(per))
///         .value(percentage())
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().padding(48.).center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_slider.png");
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
    key: DiffKey,
}

impl Slider {
    pub fn new(handler: impl FnMut(f64) + 'static) -> Self {
        Self {
            theme: None,
            value: 0.0,
            on_moved: EventHandler::new(handler),
            size: Size::fill(),
            direction: Direction::Horizontal,
            enabled: true,
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

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }
}

impl Render for Slider {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, slider);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let mut status = use_state(SliderStatus::default);
        let mut clicking = use_state(|| false);
        let mut size = use_state(Area::default);

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if status() == SliderStatus::Hovering && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let direction_is_vertical = self.direction == Direction::Vertical;
        let value = self.value.clamp(0.0, 100.0);
        let on_moved = self.on_moved.clone();

        let on_key_down = {
            let on_moved = self.on_moved.clone();
            move |e: Event<KeyboardEventData>| match e.code {
                Code::ArrowLeft if !direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value - 4.0).clamp(0.0, 100.0));
                }
                Code::ArrowRight if !direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value + 4.0).clamp(0.0, 100.0));
                }
                Code::ArrowUp if direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value + 4.0).clamp(0.0, 100.0));
                }
                Code::ArrowDown if direction_is_vertical => {
                    e.stop_propagation();
                    on_moved.call((value - 4.0).clamp(0.0, 100.0));
                }
                _ => {}
            }
        };

        let on_pointer_enter = move |_| {
            Cursor::set(CursorIcon::Pointer);
            *status.write() = SliderStatus::Hovering;
        };

        let on_pointer_leave = move |_| {
            Cursor::set(CursorIcon::default());
            *status.write() = SliderStatus::Idle;
        };

        let on_pointer_down = {
            let on_moved = self.on_moved.clone();
            move |e: Event<PointerEventData>| {
                focus.request_focus();
                clicking.set(true);
                e.stop_propagation();
                let coordinates = e.element_location();
                let percentage = if direction_is_vertical {
                    let y = coordinates.y - 8.0;
                    100. - (y / (size.read().height() as f64 - 15.0) * 100.0)
                } else {
                    let x = coordinates.x - 8.0;
                    x / (size.read().width() as f64 - 15.) * 100.0
                };
                let percentage = percentage.clamp(0.0, 100.0);

                on_moved.call(percentage);
            }
        };

        let on_global_mouse_up = move |_| {
            clicking.set(false);
        };

        let on_global_mouse_move = move |e: Event<MouseEventData>| {
            e.stop_propagation();
            if *clicking.peek() {
                let coordinates = e.global_location;
                let percentage = if direction_is_vertical {
                    let y = coordinates.y - size.read().min_y() as f64 - 8.0;
                    100. - (y / (size.read().height() as f64 - 15.0) * 100.0)
                } else {
                    let x = coordinates.x - size.read().min_x() as f64 - 8.0;
                    x / (size.read().width() as f64 - 15.) * 100.0
                };
                let percentage = percentage.clamp(0.0, 100.0);

                on_moved.call(percentage);
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

        let (
            slider_width,
            slider_height,
            track_width,
            track_height,
            thumb_offset_x,
            thumb_offset_y,
            thumb_main_align,
            padding,
        ) = if direction_is_vertical {
            (
                Size::px(6.),
                self.size.clone(),
                Size::px(6.),
                Size::func_data(
                    move |ctx| Some(value as f32 / 100. * (ctx.parent - 15.)),
                    &(value as i32),
                ),
                -6.,
                3.,
                Alignment::end(),
                (0., 8.),
            )
        } else {
            (
                self.size.clone(),
                Size::px(6.),
                Size::func_data(
                    move |ctx| Some(value as f32 / 100. * (ctx.parent - 15.)),
                    &(value as i32),
                ),
                Size::px(6.),
                -3.,
                -6.,
                Alignment::start(),
                (8., 0.),
            )
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
            .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
            .maybe(self.enabled, |rect| {
                rect.on_key_down(on_key_down)
                    .on_pointer_enter(on_pointer_enter)
                    .on_pointer_leave(on_pointer_leave)
                    .on_pointer_down(on_pointer_down)
                    .on_global_mouse_move(on_global_mouse_move)
                    .on_global_mouse_up(on_global_mouse_up)
            })
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
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
