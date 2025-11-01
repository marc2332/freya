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

/// A container that expands vertically on press.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna.";
///
/// fn app() -> Element {
///     rect()
///        .center()
///        .expanded()
///        .spacing(4.)
///        .children_iter((0..2).map(|_| {
///            accordion()
///                .header("Click to expand!")
///                .content(LOREM_IPSUM)
///                .into()
///        }))
///        .into()
/// }
///
/// # use freya_testing::prelude::*;
/// # use std::time::Duration;
/// # launch_doc_hook(|| {
/// #   rect().child(app()).into()
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
#[derive(Clone, PartialEq)]
pub struct Accordion {
    header: Option<Element>,
    content: Option<Element>,
    pub(crate) theme: Option<AccordionThemePartial>,
}

impl Accordion {
    pub fn header<C: Into<Element>>(mut self, header: C) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn content<C: Into<Element>>(mut self, content: C) -> Self {
        self.content = Some(content.into());
        self
    }
}

impl Render for Accordion {
    fn render(self: &Accordion) -> Element {
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
                    .overflow_mode(OverflowMode::Clip)
                    .visible_height(VisibleSize::inner_percent(clip_percent))
                    .maybe_child(self.content.clone()),
            )
            .into()
    }
}

pub fn accordion() -> Accordion {
    Accordion {
        header: None,
        content: None,
        theme: None,
    }
}
