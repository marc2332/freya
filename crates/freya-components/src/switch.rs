use accesskit::Toggled;
use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::{
    alignment::Alignment,
    gaps::Gaps,
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
};

define_theme! {
    for = Switch;
    theme_field = theme_colors;

    %[component]
    pub SwitchColors {
        %[fields]
        background: Color,
        thumb_background: Color,
        toggled_background: Color,
        toggled_thumb_background: Color,
        focus_border_fill: Color,
    }
}

define_theme! {
    for = Switch;
    theme_field = theme_layout;

    %[component]
    pub SwitchLayout {
        %[fields]
        margin: Gaps,
        width: f32,
        height: f32,
        padding: f32,
        thumb_size: f32,
        toggled_thumb_size: f32,
        pressed_thumb_size_offset: f32,
        thumb_offset: f32,
        toggled_thumb_offset: f32,
    }
}

#[derive(Clone, PartialEq)]
pub enum SwitchLayoutVariant {
    Normal,
    Expanded,
}

/// Toggle between `true` and `false`.
///
/// Commonly used for enabled/disabled scenarios.
///
/// Example: light/dark theme.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut toggled = use_state(|| false);
///
///     Switch::new()
///         .toggled(toggled())
///         .on_toggle(move |_| toggled.toggle())
/// }
/// # // TOGGLED
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(Switch::new().toggled(true))
/// # }, "./images/gallery_toggled_switch.png").render();
/// #
/// # // NOT TOGGLED
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(Switch::new().toggled(false))
/// # }, "./images/gallery_not_toggled_switch.png").render();
/// ```
/// # Preview
///
/// | Toggled       | Not Toggled   |
/// | ------------- | ------------- |
/// | ![Switch Toggled Demo][gallery_toggled_switch] | ![Switch Not Toggled Demo][gallery_not_toggled_switch] |
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!(
        "gallery_toggled_switch",
        "images/gallery_toggled_switch.png"
    ),
    doc = embed_doc_image::embed_image!("gallery_not_toggled_switch", "images/gallery_not_toggled_switch.png")
)]
#[derive(Clone, PartialEq)]
pub struct Switch {
    pub(crate) theme_colors: Option<SwitchColorsThemePartial>,
    pub(crate) theme_layout: Option<SwitchLayoutThemePartial>,
    layout_variant: SwitchLayoutVariant,
    toggled: Readable<bool>,
    on_toggle: Option<EventHandler<()>>,
    enabled: bool,
    key: DiffKey,
}

impl KeyExt for Switch {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Switch {
    pub fn new() -> Self {
        Self {
            toggled: false.into(),
            on_toggle: None,
            theme_colors: None,
            theme_layout: None,
            layout_variant: SwitchLayoutVariant::Normal,
            enabled: true,
            key: DiffKey::None,
        }
    }

    pub fn toggled(mut self, toggled: impl Into<Readable<bool>>) -> Self {
        self.toggled = toggled.into();
        self
    }

    pub fn on_toggle(mut self, on_toggle: impl Into<EventHandler<()>>) -> Self {
        self.on_toggle = Some(on_toggle.into());
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn layout_variant(mut self, layout_variant: impl Into<SwitchLayoutVariant>) -> Self {
        self.layout_variant = layout_variant.into();
        self
    }

    pub fn theme_colors(mut self, theme: SwitchColorsThemePartial) -> Self {
        self.theme_colors = Some(theme);
        self
    }

    pub fn theme_layout(mut self, theme: SwitchLayoutThemePartial) -> Self {
        self.theme_layout = Some(theme);
        self
    }

    /// Shortcut for [Self::layout_variant] and [SwitchLayoutVariant::Expanded].
    pub fn expanded(self) -> Self {
        self.layout_variant(SwitchLayoutVariant::Expanded)
    }
}

impl Component for Switch {
    fn render(self: &Switch) -> impl IntoElement {
        let theme_colors = get_theme!(&self.theme_colors, SwitchColorsThemePreference, "switch");
        let theme_layout = match self.layout_variant {
            SwitchLayoutVariant::Normal => get_theme!(
                &self.theme_layout,
                SwitchLayoutThemePreference,
                "switch_layout"
            ),
            SwitchLayoutVariant::Expanded => get_theme!(
                &self.theme_layout,
                SwitchLayoutThemePreference,
                "expanded_switch_layout"
            ),
        };

        let mut hovering = use_state(|| false);
        let mut pressing = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let toggled = *self.toggled.read();

        let anim_toggle = use_animation_with_dependencies(
            &(theme_colors.clone(), theme_layout.clone(), toggled),
            |conf, (switch_colors, switch_layout, toggled)| {
                conf.on_creation(OnCreation::Finish);
                conf.on_change(OnChange::Finish);

                let value = (
                    AnimNum::new(
                        switch_layout.thumb_offset,
                        switch_layout.toggled_thumb_offset,
                    )
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
                    AnimNum::new(switch_layout.thumb_size, switch_layout.toggled_thumb_size)
                        .time(300)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimColor::new(switch_colors.background, switch_colors.toggled_background)
                        .time(300)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimColor::new(
                        switch_colors.thumb_background,
                        switch_colors.toggled_thumb_background,
                    )
                    .time(300)
                    .function(Function::Expo)
                    .ease(Ease::Out),
                );

                if *toggled {
                    value
                } else {
                    value.into_reversed()
                }
            },
        );

        let anim_press = use_animation_with_dependencies(&pressing(), move |conf, pressing| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);
            let anim = AnimNum::new(0.0, theme_layout.pressed_thumb_size_offset)
                .time(150)
                .function(Function::Expo)
                .ease(Ease::Out);
            if *pressing {
                anim
            } else {
                anim.into_reversed()
            }
        });
        let press_size = anim_press.get().value();

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if hovering() && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .width(2.)
                .alignment(BorderAlignment::Inner)
                .fill(theme_colors.focus_border_fill.mul_if(!self.enabled, 0.9))
        } else {
            Border::new()
        };
        let (offset_x, size, background, thumb) = anim_toggle.get().value();

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .a11y_role(AccessibilityRole::Switch)
            .a11y_builder(|builder| builder.set_toggled(Toggled::from(toggled)))
            .width(Size::px(theme_layout.width))
            .height(Size::px(theme_layout.height))
            .padding(Gaps::new_all(theme_layout.padding))
            .main_align(Alignment::center())
            .offset_x(offset_x - press_size / 2.0)
            .corner_radius(CornerRadius::new_all(50.))
            .background(background.mul_if(!self.enabled, 0.85))
            .border(border)
            .maybe(self.enabled, |rect| {
                rect.on_press({
                    let on_toggle = self.on_toggle.clone();
                    move |_| {
                        if let Some(on_toggle) = &on_toggle {
                            on_toggle.call(())
                        }
                        focus.request_focus();
                    }
                })
                .on_pointer_down(move |e: Event<PointerEventData>| {
                    if matches!(e.data(), PointerEventData::Touch(_)) {
                        pressing.set(true);
                    }
                })
            })
            .on_global_pointer_press(move |_| pressing.set_if_modified(false))
            .on_pointer_enter(move |_| {
                hovering.set(true);
                if enabled() {
                    Cursor::set(CursorIcon::Pointer);
                } else {
                    Cursor::set(CursorIcon::NotAllowed);
                }
            })
            .on_pointer_leave(move |_| {
                if hovering() {
                    Cursor::set(CursorIcon::default());
                    hovering.set(false);
                }
                pressing.set_if_modified(false);
            })
            .child(
                rect()
                    .width(Size::px(size + press_size))
                    .height(Size::px(size + press_size))
                    .background(thumb.mul_if(!self.enabled, 0.85))
                    .corner_radius(CornerRadius::new_all(50.)),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
