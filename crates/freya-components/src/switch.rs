use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::{
    alignment::Alignment,
    gaps::Gaps,
    size::Size,
};

use crate::{
    get_theme,
    theming::component_themes::SwitchThemePartial,
};

/// Toggle between `true` and `false`.
///
/// Commonly used for enabled/disabled scenarios.
///
/// Example: light/dark theme.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut toggled = use_state(|| false);
///
///     Switch::new()
///         .toggled(toggled())
///         .on_toggle(move |_| toggled.toggle())
///         .into()
/// }
/// # // TOGGLED
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(Switch::new().toggled(true)).into()
/// # }, (250., 250.).into(), "./images/gallery_toggled_switch.png");
/// #
/// # // NOT TOGGLED
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(Switch::new().toggled(false)).into()
/// # }, (250., 250.).into(), "./images/gallery_not_toggled_switch.png");
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
    pub(crate) theme: Option<SwitchThemePartial>,
    toggled: ReadState<bool>,
    on_toggle: Option<EventHandler<()>>,
    enabled: bool,
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
            theme: None,
            enabled: true,
        }
    }

    pub fn toggled(mut self, toggled: impl Into<ReadState<bool>>) -> Self {
        self.toggled = toggled.into();
        self
    }

    pub fn on_toggle(mut self, on_toggle: impl FnMut(()) + 'static) -> Self {
        self.on_toggle = Some(EventHandler::new(on_toggle));
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }
}

impl Render for Switch {
    fn render(self: &Switch) -> Element {
        let theme = get_theme!(&self.theme, switch);
        let mut hovering = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let toggled = *self.toggled.read();

        let animation = use_animation_with_dependencies(
            &(theme.clone(), toggled),
            |conf, (switch_theme, toggled)| {
                conf.on_creation(OnCreation::Finish);
                conf.on_change(OnChange::Rerun);

                let value = (
                    AnimNum::new(2., 22.)
                        .time(300)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimNum::new(14., 18.)
                        .time(300)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimColor::new(switch_theme.background, switch_theme.toggled_background)
                        .time(300)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimColor::new(
                        switch_theme.thumb_background,
                        switch_theme.toggled_thumb_background,
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
                .fill(theme.focus_border_fill.mul_if(!self.enabled, 0.9))
        } else {
            Border::new()
        };
        let (offset_x, size, background, thumb) = animation.get().value();

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .width(Size::px(48.))
            .height(Size::px(25.))
            .padding(Gaps::new_all(4.0))
            .main_align(Alignment::center())
            .offset_x(offset_x)
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
                .on_pointer_enter(move |_| {
                    Cursor::set(CursorIcon::Pointer);
                    hovering.set(true);
                })
            })
            .on_pointer_leave(move |_| {
                if hovering() {
                    Cursor::set(CursorIcon::default());
                    hovering.set(false);
                }
            })
            .child(
                rect()
                    .width(Size::px(size))
                    .height(Size::px(size))
                    .background(thumb.mul_if(!self.enabled, 0.85))
                    .corner_radius(CornerRadius::new_all(50.)),
            )
            .into()
    }
}
