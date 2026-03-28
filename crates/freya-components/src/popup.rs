use freya_animation::prelude::*;
use freya_core::prelude::*;
use torin::{
    prelude::{
        Alignment,
        Position,
    },
    size::Size,
};

use crate::{
    get_theme,
    theming::component_themes::{
        PopupTheme,
        PopupThemePartial,
    },
};

/// Popup background wrapper.
#[derive(Clone, PartialEq)]
pub struct PopupBackground {
    pub children: Element,
    pub on_press: EventHandler<Event<PressEventData>>,
    pub background: Color,
}

impl PopupBackground {
    pub fn new(
        children: Element,
        on_press: impl Into<EventHandler<Event<PressEventData>>>,
        background: Color,
    ) -> Self {
        Self {
            children,
            on_press: on_press.into(),
            background,
        }
    }
}

impl Component for PopupBackground {
    fn render(&self) -> impl IntoElement {
        let on_press = self.on_press.clone();

        rect()
            .child(
                rect()
                    .on_press(on_press)
                    .position(Position::new_global().top(0.).left(0.))
                    .height(Size::window_percent(100.))
                    .width(Size::window_percent(100.))
                    .background(self.background),
            )
            .child(
                rect()
                    .position(Position::new_global().top(0.).left(0.))
                    .height(Size::window_percent(100.))
                    .width(Size::window_percent(100.))
                    .center()
                    .child(self.children.clone()),
            )
    }
}

/// Floating popup / dialog.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut show_popup = use_state(|| true);
///
///     rect()
///         .child(
///             Popup::new()
///                 .show(show_popup())
///                 .width(Size::px(250.))
///                 .on_close_request(move |_| show_popup.set(false))
///                 .child(PopupTitle::new("Title".to_string()))
///                 .child(PopupContent::new().child("Hello, World!"))
///                 .child(
///                     PopupButtons::new().child(
///                         Button::new()
///                             .on_press(move |_| show_popup.set(false))
///                             .expanded()
///                             .filled()
///                             .child("Accept"),
///                     ),
///                 ),
///         )
///         .child(
///             Button::new()
///                 .child("Open")
///                 .on_press(move |_| show_popup.toggle()),
///         )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(
/// #      app()
/// #   )
/// # }, "./images/gallery_popup.png").with_scale_factor(0.8).with_hook(|test| {
/// #   test.poll(std::time::Duration::from_millis(10), std::time::Duration::from_millis(500));
/// # }).render();
/// ```
///
/// # Preview
/// ![Popup Preview][popup]
#[doc(alias = "alert")]
#[doc(alias = "dialog")]
#[doc(alias = "window")]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("popup", "images/gallery_popup.png"),
)]
#[derive(Clone, PartialEq)]
pub struct Popup {
    pub(crate) theme: Option<PopupThemePartial>,
    children: Vec<Element>,
    show: Readable<bool>,
    on_close_request: Option<EventHandler<()>>,
    close_on_escape_key: bool,
    width: Size,
    key: DiffKey,
}

impl KeyExt for Popup {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Default for Popup {
    fn default() -> Self {
        Self::new()
    }
}

impl Popup {
    pub fn new() -> Self {
        Self {
            theme: None,
            children: vec![],
            show: true.into(),
            on_close_request: None,
            close_on_escape_key: true,
            width: Size::px(500.),
            key: DiffKey::None,
        }
    }

    pub fn show(mut self, show: impl Into<Readable<bool>>) -> Self {
        self.show = show.into();
        self
    }

    pub fn on_close_request(mut self, on_close_request: impl Into<EventHandler<()>>) -> Self {
        self.on_close_request = Some(on_close_request.into());
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }
}

impl ChildrenExt for Popup {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for Popup {
    fn render(&self) -> impl IntoElement {
        let show = *self.show.read();

        let bg_animation = use_animation_with_dependencies(&show, |conf, show| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);

            let value = AnimColor::new((0, 0, 0, 0), (0, 0, 0, 150)).time(150);

            if *show { value } else { value.into_reversed() }
        });

        // Depends on `show` to restart on reopen
        let content_animation = use_animation_with_dependencies(&show, |conf, _| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);

            (
                AnimNum::new(0.85, 1.)
                    .time(250)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimNum::new(0.2, 1.)
                    .time(250)
                    .ease(Ease::Out)
                    .function(Function::Expo),
            )
        });

        let should_render = show || *bg_animation.is_running().read();

        let PopupTheme { background, color } = get_theme!(&self.theme, popup);

        let request_to_close = {
            let handler = self.on_close_request.clone();
            move || {
                if let Some(h) = &handler {
                    h.call(());
                }
            }
        };

        let on_global_key_down = {
            let close = self.close_on_escape_key;
            let req = request_to_close.clone();
            move |e: Event<KeyboardEventData>| {
                if close && e.key == Key::Named(NamedKey::Escape) {
                    req();
                }
            }
        };

        rect()
            .layer(Layer::Overlay)
            .position(Position::new_global())
            .maybe_child(should_render.then(|| {
                let bg_color = bg_animation.get().value();

                let (scale, opacity) = &*content_animation.read();

                let (scale, opacity) = if show {
                    (scale.value(), opacity.value())
                } else {
                    (1., 0.)
                };

                PopupBackground::new(
                    rect()
                        .a11y_role(AccessibilityRole::Dialog)
                        .scale((scale, scale))
                        .opacity(opacity)
                        .corner_radius(12.)
                        .background(background)
                        .color(color)
                        .shadow(Shadow::new().y(4.).blur(5.).color((0, 0, 0, 30)))
                        .width(self.width.clone())
                        .height(Size::auto())
                        .spacing(4.)
                        .padding(8.)
                        .on_global_key_down(on_global_key_down)
                        .children(self.children.clone())
                        .into(),
                    move |_| {
                        request_to_close();
                    },
                    bg_color,
                )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Popup title.
#[derive(PartialEq)]
pub struct PopupTitle {
    text: Readable<String>,
}

impl PopupTitle {
    pub fn new(text: impl Into<Readable<String>>) -> Self {
        Self { text: text.into() }
    }
}

impl Component for PopupTitle {
    fn render(&self) -> impl IntoElement {
        rect().font_size(18.).padding(8.).child(
            label()
                .a11y_role(AccessibilityRole::TitleBar)
                .width(Size::fill())
                .text(self.text.read().to_string()),
        )
    }
}

/// Popup content wrapper.
#[derive(Clone, PartialEq)]
pub struct PopupContent {
    children: Vec<Element>,
}
impl Default for PopupContent {
    fn default() -> Self {
        Self::new()
    }
}

impl PopupContent {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl ChildrenExt for PopupContent {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for PopupContent {
    fn render(&self) -> impl IntoElement {
        rect()
            .font_size(15.)
            .padding(8.)
            .children(self.children.clone())
    }
}

/// Popup buttons container.
#[derive(Clone, PartialEq)]
pub struct PopupButtons {
    pub children: Vec<Element>,
}

impl Default for PopupButtons {
    fn default() -> Self {
        Self::new()
    }
}

impl PopupButtons {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl ChildrenExt for PopupButtons {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for PopupButtons {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .main_align(Alignment::End)
            .padding(8.)
            .spacing(4.)
            .horizontal()
            .children(self.children.clone())
    }
}
