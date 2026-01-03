use freya_animation::{
    easing::Function,
    hook::{
        AnimatedValue,
        Ease,
        OnChange,
        OnCreation,
        ReadAnimatedValue,
        use_animation,
    },
    prelude::AnimNum,
};
use freya_core::prelude::*;
use freya_edit::Clipboard;
use torin::prelude::{
    Alignment,
    Area,
    Position,
    Size,
};

use crate::{
    button::Button,
    context_menu::ContextMenu,
    get_theme,
    menu::{
        Menu,
        MenuButton,
    },
    theming::component_themes::ColorPickerThemePartial,
};

/// HSV-based gradient color picker.
///
/// Click preview to open a popup with a saturation/value gradient area and a hue bar.
#[derive(Clone, PartialEq)]
pub struct ColorPicker {
    pub(crate) theme: Option<ColorPickerThemePartial>,
    value: Color,
    on_change: EventHandler<Color>,
    width: Size,
    key: DiffKey,
}

impl KeyExt for ColorPicker {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ColorPicker {
    pub fn new(on_change: impl Into<EventHandler<Color>>) -> Self {
        Self {
            theme: None,
            value: Color::WHITE,
            on_change: on_change.into(),
            width: Size::px(220.),
            key: DiffKey::None,
        }
    }

    pub fn theme(mut self, theme: ColorPickerThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn value(mut self, value: Color) -> Self {
        self.value = value;
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }
}

impl Render for ColorPicker {
    fn render(&self) -> impl IntoElement {
        let mut open = use_state(|| false);
        let mut color = use_state(|| self.value);
        let mut pressing = use_state(|| false);
        let mut pressing_hue = use_state(|| false);
        let mut area = use_state(Area::default);
        let mut hue_area = use_state(Area::default);

        let is_open = open();

        let preview = rect()
            .width(Size::px(40.))
            .height(Size::px(24.))
            .corner_radius(4.)
            .background(self.value)
            .on_press(move |_| {
                open.toggle();
            });

        let theme = get_theme!(&self.theme, color_picker);
        let hue_bar = rect()
            .height(Size::px(18.))
            .width(Size::fill())
            .corner_radius(4.)
            .on_sized(move |e: Event<SizedEventData>| hue_area.set(e.area))
            .background_linear_gradient(
                LinearGradient::new()
                    .angle(-90.)
                    .stop(((255, 0, 0), 0.))
                    .stop(((255, 255, 0), 16.))
                    .stop(((0, 255, 0), 33.))
                    .stop(((0, 255, 255), 50.))
                    .stop(((0, 0, 255), 66.))
                    .stop(((255, 0, 255), 83.))
                    .stop(((255, 0, 0), 100.)),
            );

        let sv_area = rect()
            .height(Size::px(140.))
            .width(Size::fill())
            .corner_radius(4.)
            .overflow(Overflow::Clip)
            .child(
                rect()
                    .expanded()
                    .background_linear_gradient(
                        // left: white -> right: hue color
                        LinearGradient::new()
                            .angle(-90.)
                            .stop(((255, 255, 255), 0.))
                            .stop((Color::from_hsv(color.read().to_hsv().h, 1.0, 1.0), 100.)),
                    )
                    .child(
                        rect()
                            .position(Position::new_absolute())
                            .expanded()
                            .background_linear_gradient(
                                // top: transparent -> bottom: black
                                LinearGradient::new()
                                    .stop(((255, 255, 255, 0.0), 0.))
                                    .stop(((0, 0, 0), 100.)),
                            ),
                    ),
            );

        // Minimum perceptible floor to avoid full desaturation/black when dragging
        const MIN_S: f32 = 0.07;
        const MIN_V: f32 = 0.07;

        let on_sv_pointer_down = {
            let on_change = self.on_change.clone();
            move |e: Event<PointerEventData>| {
                pressing.set(true);
                let coords = e.element_location();
                let area = area.read().to_f64();
                let rel_x = (((coords.x - area.min_x()) / area.width()).clamp(0., 1.)) as f32;
                let rel_y = (((coords.y - area.min_y()) / area.height())
                    .clamp(MIN_V as f64, 1. - MIN_V as f64)) as f32;
                let sat = rel_x.max(MIN_S);
                let v = (1.0 - rel_y).clamp(MIN_V, 1.0 - MIN_V);
                color.with_mut(|mut color| *color = color.with_s(sat).with_v(v));
                on_change.call(color());
            }
        };

        let on_hue_pointer_down = {
            let on_change = self.on_change.clone();
            move |e: Event<PointerEventData>| {
                pressing_hue.set(true);
                let coords = e.global_location();
                let area = hue_area.read().to_f64();
                let rel_x = ((coords.x - area.min_x()) / area.width()).clamp(0.01, 1.) as f32;
                color.with_mut(|mut color| *color = color.with_h(rel_x * 360.0));
                on_change.call(color());
            }
        };

        let on_global_mouse_move = {
            let on_change = self.on_change.clone();
            move |e: Event<MouseEventData>| {
                if *pressing.read() {
                    let coords = e.global_location;
                    let area = area.read().to_f64();
                    let rel_x = (((coords.x - area.min_x()) / area.width()).clamp(0., 1.)) as f32;
                    let rel_y = (((coords.y - area.min_y()) / area.height())
                        .clamp(MIN_V as f64, 1. - MIN_V as f64))
                        as f32;
                    let sat = rel_x.max(MIN_S);
                    let v = (1.0 - rel_y).clamp(MIN_V, 1.0 - MIN_V);
                    color.with_mut(|mut color| *color = color.with_s(sat).with_v(v));
                    on_change.call(color());
                } else if *pressing_hue.read() {
                    let coords = e.global_location;
                    let area = hue_area.read().to_f64();
                    let rel_x = ((coords.x - area.min_x()) / area.width()).clamp(0.01, 1.) as f32;
                    color.with_mut(|mut color| *color = color.with_h(rel_x * 360.0));
                    on_change.call(color());
                }
            }
        };

        let on_global_mouse_up = move |_| {
            // Only close the popup if it wasnt being pressed and it is open
            if is_open && !pressing() && !pressing_hue() {
                open.set(false);
            }
            pressing.set_if_modified(false);
            pressing_hue.set_if_modified(false);
        };

        let animation = use_animation(move |conf| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Finish);

            let scale = AnimNum::new(0.8, 1.)
                .time(200)
                .ease(Ease::Out)
                .function(Function::Expo);
            let opacity = AnimNum::new(0., 1.)
                .time(200)
                .ease(Ease::Out)
                .function(Function::Expo);

            if open() {
                (scale, opacity)
            } else {
                (scale, opacity).into_reversed()
            }
        });

        let (scale, opacity) = animation.read().value();

        let popup = rect()
            .on_global_mouse_move(on_global_mouse_move)
            .on_global_mouse_up(on_global_mouse_up)
            .width(self.width.clone())
            .padding(8.)
            .corner_radius(6.)
            .background(theme.background)
            .border(
                Border::new()
                    .fill(theme.border_fill)
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            )
            .color(theme.color)
            .spacing(8.)
            .shadow(Shadow::new().x(0.).y(2.).blur(8.).color((0, 0, 0, 0.1)))
            .child(
                rect()
                    .on_sized(move |e: Event<SizedEventData>| area.set(e.area))
                    .on_pointer_down(on_sv_pointer_down)
                    .child(sv_area),
            )
            .child(
                rect()
                    .height(Size::px(18.))
                    .on_pointer_down(on_hue_pointer_down)
                    .child(hue_bar),
            )
            .child({
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.read().r(),
                    color.read().g(),
                    color.read().b()
                );

                rect()
                    .horizontal()
                    .width(Size::fill())
                    .main_align(Alignment::center())
                    .spacing(8.)
                    .child(
                        Button::new()
                            .on_press(move |e: Event<PressEventData>| {
                                e.stop_propagation();
                                e.prevent_default();
                                if ContextMenu::is_open() {
                                    ContextMenu::close();
                                } else {
                                    ContextMenu::open(
                                        Menu::new()
                                            .child(
                                                MenuButton::new()
                                                    .on_press(move |e: Event<PressEventData>| {
                                                        e.stop_propagation();
                                                        e.prevent_default();
                                                        ContextMenu::close();
                                                        let _ =
                                                            Clipboard::set(color().to_rgb_string());
                                                    })
                                                    .child("Copy as RGB"),
                                            )
                                            .child(
                                                MenuButton::new()
                                                    .on_press(move |e: Event<PressEventData>| {
                                                        e.stop_propagation();
                                                        e.prevent_default();
                                                        ContextMenu::close();
                                                        let _ =
                                                            Clipboard::set(color().to_hex_string());
                                                    })
                                                    .child("Copy as HEX"),
                                            ),
                                    )
                                }
                            })
                            .compact()
                            .child(hex),
                    )
            });

        rect().horizontal().spacing(8.).child(preview).child(
            rect()
                .width(Size::px(0.))
                .height(Size::px(0.))
                .opacity(opacity)
                .maybe(opacity > 0., |el| {
                    el.child(rect().scale(scale).child(popup))
                }),
        )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
