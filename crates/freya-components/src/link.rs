use freya_core::prelude::*;
use freya_router::prelude::{
    NavigationTarget,
    Navigator,
};

use crate::{
    get_theme,
    theming::component_themes::LinkThemePartial,
    tooltip::{
        Tooltip,
        TooltipContainer,
    },
};

/// Tooltip configuration for the [`Link`] component.
#[derive(Clone, PartialEq)]
pub enum LinkTooltip {
    /// No tooltip at all.
    None,
    /// Default tooltip.
    ///
    /// - For a route, this is the same as [`None`](LinkTooltip::None).
    /// - For a URL, this is the value of that URL.
    Default,
    /// Custom tooltip to always show.
    Custom(String),
}

#[derive(PartialEq)]
pub struct Link {
    /// Theme override.
    pub(crate) theme: Option<LinkThemePartial>,
    /// The route or external URL string to navigate to.
    to: NavigationTarget,
    /// Inner children for the Link.
    children: Vec<Element>,
    /// A text hint to show when hovering over the link.
    tooltip: LinkTooltip,
    /// Key for the component.
    key: DiffKey,
}

impl ChildrenExt for Link {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Link {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Link {
    pub fn new(to: impl Into<NavigationTarget>) -> Self {
        Self {
            to: to.into(),
            children: Vec::new(),
            tooltip: LinkTooltip::Default,
            theme: None,
            key: DiffKey::None,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Into<LinkTooltip>) -> Self {
        self.tooltip = tooltip.into();
        self
    }
}

impl Component for Link {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, link);
        let mut is_hovering = use_state(|| false);

        let url = if let NavigationTarget::External(ref url) = self.to {
            Some(url.clone())
        } else {
            None
        };

        let on_pointer_enter = move |_| {
            is_hovering.set(true);
        };

        let on_pointer_leave = move |_| {
            is_hovering.set(false);
        };

        let on_press = {
            let to = self.to.clone();
            let url = url.clone();
            move |_| {
                // Open the url if there is any
                // otherwise change the freya router route
                if let Some(url) = &url {
                    let _ = open::that(url);
                } else {
                    Navigator::get().push(to.clone());
                }
            }
        };

        let color = if *is_hovering.read() {
            Some(theme.color)
        } else {
            None
        };

        let tooltip_text = match &self.tooltip {
            LinkTooltip::Default => url.clone(),
            LinkTooltip::None => None,
            LinkTooltip::Custom(str) => Some(str.clone()),
        };

        let link = rect()
            .on_press(on_press)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .maybe_some(color, |rect, color| rect.color(color))
            .children(self.children.clone());

        if let Some(tooltip_text) = tooltip_text {
            TooltipContainer::new(Tooltip::new(tooltip_text))
                .child(link)
                .into_element()
        } else {
            link.into()
        }
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
