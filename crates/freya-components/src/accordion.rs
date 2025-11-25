use freya_animation::prelude::{
    AnimNum,
    Ease,
    Function,
    use_animation,
};
use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    prelude::VisibleSize,
};

use crate::{
    get_theme,
    theming::component_themes::AccordionThemePartial,
};

/// A container that expands/collapses vertically when pressed.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna.";
///
/// fn app() -> impl IntoElement {
///     rect()
///        .center()
///        .expanded()
///        .spacing(4.)
///        .children_iter((0..2).map(|_| {
///            Accordion::new()
///                .header("Click to expand!")
///                .child(LOREM_IPSUM)
///                .into()
///        }))
/// }
///
/// # use freya_testing::prelude::*;
/// # use std::time::Duration;
/// # launch_doc_hook(|| {
/// #   rect().child(app())
/// # }, (250., 250.).into(), "./images/gallery_accordion.png", |t| {
/// #   t.click_cursor((125., 115.));
/// #   t.poll(Duration::from_millis(1), Duration::from_millis(300));
/// #   t.sync_and_update();
/// # });
/// ```
///
/// # Preview
/// ![Accordion Preview][accordion]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("accordion", "images/gallery_accordion.png")
)]
#[derive(Clone, PartialEq, Default)]
pub struct Accordion {
    pub(crate) theme: Option<AccordionThemePartial>,
    header: Option<Element>,
    children: Vec<Element>,
}

impl Accordion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header<C: Into<Element>>(mut self, header: C) -> Self {
        self.header = Some(header.into());
        self
    }
}

impl ChildrenExt for Accordion {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Render for Accordion {
    fn render(self: &Accordion) -> impl IntoElement {
        let header = use_focus();
        let accordion_theme = get_theme!(&self.theme, accordion);
        let mut open = use_state(|| false);
        let mut animation = use_animation(move |_conf| {
            AnimNum::new(0., 100.)
                .time(300)
                .function(Function::Expo)
                .ease(Ease::Out)
        });

        let clip_percent = animation.get().value();

        rect()
            .a11y_id(header.a11y_id())
            .a11y_role(AccessibilityRole::Header)
            .a11y_focusable(true)
            .corner_radius(CornerRadius::new_all(8.))
            .padding(Gaps::new_all(8.))
            .color(accordion_theme.color)
            .background(accordion_theme.background)
            .border(
                Border::new()
                    .fill(accordion_theme.border_fill)
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            )
            .on_pointer_enter(move |_| {
                Cursor::set(CursorIcon::Pointer);
            })
            .on_pointer_leave(move |_| {
                Cursor::set(CursorIcon::default());
            })
            .on_press(move |_| {
                if open.toggled() {
                    animation.start();
                } else {
                    animation.reverse();
                }
            })
            .maybe_child(self.header.clone())
            .child(
                rect()
                    .a11y_role(AccessibilityRole::Region)
                    .a11y_builder(|b| {
                        b.set_labelled_by([header.a11y_id()]);
                        if !open() {
                            b.set_hidden();
                        }
                    })
                    .overflow(Overflow::Clip)
                    .visible_height(VisibleSize::inner_percent(clip_percent))
                    .children(self.children.clone()),
            )
    }
}
