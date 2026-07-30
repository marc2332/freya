use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    prelude::Alignment,
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    icons::arrow::ArrowIcon,
    scrollviews::{
        VirtualItem,
        VirtualScrollView,
    },
};

define_theme! {
    // The generated setter trait goes on `TreeItem`, not on `Tree`: per-row dress is what a caller
    // overrides, and `Tree` is generic over its row builder so the trait cannot be implemented for
    // it by name anyway.
    for = TreeItem ; theme_field = theme ;
    %[component]
    pub Tree {
        %[fields]
        background: Color,
        color: Color,
        /// The disclosure arrow. Separate from `color` so it can sit back from the label it
        /// belongs to, which is how most trees draw it.
        arrow_fill: Color,
        item_background: Color,
        hover_item_background: Color,
        /// A selected item. Set it to `item_background` to opt a tree out of selection dress
        /// entirely, the way `TableRow` opts out of hover.
        selected_item_background: Color,
        selected_item_color: Color,
        /// The vertical rule marking each level of indentation. Set it transparent for a tree
        /// that indents without guides.
        guide_fill: Color,
        /// Width of one level of indentation, and therefore the guide spacing.
        indent: f32,
        item_height: f32,
        item_padding: Gaps,
        corner_radius: CornerRadius,
    }
}

/// Wheel axis-lock threshold: `1.0` commits a gesture to whichever axis dominates. Raise it to
/// allow more diagonal freedom before locking.
const WHEEL_AXIS_LOCK: f32 = 1.0;

/// Metrics every [`TreeItem`] reads from its [`Tree`], so indentation and row height are set once
/// on the tree rather than repeated on every row.
#[derive(Clone, Copy, PartialEq)]
pub struct TreeConfig {
    pub indent: f32,
    pub item_height: f32,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            indent: 16.,
            item_height: 24.,
        }
    }
}

/// Whether an item can be opened, and whether it is.
///
/// A leaf is `None` rather than `Some(false)`: the two differ in what the row draws, since a leaf
/// has no disclosure arrow at all and an item that *could* open keeps its arrow whichever way it is
/// pointing. Collapsing them would make an empty container look openable.
#[derive(Clone, Copy, PartialEq)]
pub enum Disclosure {
    Leaf,
    Collapsed,
    Expanded,
}

impl Disclosure {
    /// A container's state from whether it is open; a leaf is [`Disclosure::Leaf`].
    pub fn from_expanded(expanded: bool) -> Self {
        if expanded {
            Self::Expanded
        } else {
            Self::Collapsed
        }
    }

    fn rotation(self) -> f32 {
        // The shared arrow points down; a collapsed row points it at its own content.
        match self {
            Self::Expanded => 0.,
            _ => -90.,
        }
    }
}

/// One row of a [`Tree`]: indentation and guides for its depth, a disclosure arrow, and whatever
/// the caller puts in it.
///
/// The row draws structure and state; it holds neither. **Which item is selected, which are
/// expanded, and which rows exist at all belong to the caller** — the same division `Table` makes,
/// and here it is what allows a tree over data that is fetched as it opens: the caller expands its
/// own set, works out the visible rows, and hands them over one at a time.
#[derive(PartialEq, Default)]
pub struct TreeItem {
    pub theme: Option<TreeThemePartial>,
    /// Levels of indentation. `0` is a root row.
    depth: usize,
    disclosure: Option<Disclosure>,
    selected: bool,
    /// Pressing the row — selecting it. Fired for a press anywhere that is not the arrow.
    on_press: Option<EventHandler<Event<PressEventData>>>,
    /// Pressing the disclosure arrow — opening or closing the row.
    on_toggle: Option<EventHandler<Event<PressEventData>>>,
    children: Vec<Element>,
    key: DiffKey,
}

impl TreeItem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Levels of indentation for this row.
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Whether this row opens, and whether it is open. Omitted, the row draws no arrow and keeps
    /// the space, so a leaf's label still lines up with its siblings'.
    pub fn disclosure(mut self, disclosure: Disclosure) -> Self {
        self.disclosure = Some(disclosure);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_press(mut self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(handler.into());
        self
    }

    /// Handle the disclosure arrow's press. It does **not** also fire `on_press`: opening a row and
    /// selecting it are different intents, and a tree whose arrow did both could not offer one
    /// without the other.
    pub fn on_toggle(mut self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_toggle = Some(handler.into());
        self
    }

    /// Dress this row on its own, over the tree's theme.
    pub fn theme(mut self, theme: TreeThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ChildrenExt for TreeItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for TreeItem {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for TreeItem {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, TreeThemePreference, "tree");
        let config = use_try_consume::<TreeConfig>().unwrap_or_default();
        let mut hovering = use_state(|| false);
        let TreeTheme {
            color,
            arrow_fill,
            item_background,
            hover_item_background,
            selected_item_background,
            selected_item_color,
            guide_fill,
            item_padding,
            corner_radius,
            ..
        } = theme;

        let background = match (self.selected, hovering()) {
            (true, _) => selected_item_background,
            (false, true) => hover_item_background,
            (false, false) => item_background,
        };
        let disclosure = self.disclosure.unwrap_or(Disclosure::Leaf);
        let on_toggle = self.on_toggle.clone();

        // One guide per level, at the tree's indent — so a row's own depth is legible without
        // counting the gap in from the edge.
        let guides = (0..self.depth).map(|_| {
            rect()
                .width(Size::px(config.indent))
                .height(Size::fill())
                .child(
                    rect()
                        .width(Size::px(1.))
                        .height(Size::fill())
                        .background(guide_fill),
                )
        });

        // The arrow keeps its slot on a leaf, so labels down one level all start at the same x.
        let arrow = rect()
            .width(Size::px(config.indent))
            .height(Size::fill())
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .maybe(!matches!(disclosure, Disclosure::Leaf), |el| {
                el.child(
                    ArrowIcon::new()
                        .rotate(disclosure.rotation())
                        .fill(arrow_fill),
                )
                // The arrow takes its own press and stops there: `Switch`'s press reaching its
                // ancestors is what makes a "press the row to toggle" wrapper fire twice, so an
                // arrow inside a pressable row has to consume its own.
                .map(on_toggle, |el, on_toggle| {
                    el.on_pointer_down(move |e: Event<PointerEventData>| e.stop_propagation())
                        .on_press(move |e| on_toggle.call(e))
                })
            });

        rect()
            .height(Size::px(config.item_height))
            // Hugs its own content, so a long row exceeds the viewport — which is what the scroll
            // view measures as overflow, and therefore what makes the horizontal pan possible. A
            // row sized `fill` clamps itself to the visible width, and then nothing ever overflows.
            .width(Size::Inner)
            .horizontal()
            .cross_align(Alignment::Center)
            .padding(item_padding)
            .corner_radius(corner_radius)
            .background(background)
            .color(if self.selected {
                selected_item_color
            } else {
                color
            })
            .on_pointer_enter(move |_| hovering.set(true))
            .on_pointer_leave(move |_| hovering.set(false))
            .map(self.on_press.clone(), |el, on_press| {
                el.on_press(move |e| on_press.call(e))
            })
            .children(guides)
            .child(arrow)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// A virtualized tree over a **flat list of visible rows**.
///
/// The flat list is the whole design. A tree that owned a node hierarchy would have to be handed
/// every node up front, which is exactly what a large document cannot afford; taking the rows that
/// are currently visible instead lets the caller expand its own set and materialize a node's
/// children only when it opens. So `length` is "how many rows are on screen if you scrolled through
/// all of them", and the builder is asked for one row at a time, as [`VirtualScrollView`] reaches it.
///
/// Selection and keyboard movement are the caller's, as they are for [`Table`](crate::Table): both
/// are about which row means something, which the tree cannot know.
pub struct Tree<D, B: Fn(VirtualItem, &D) -> Element> {
    pub theme: Option<TreeThemePartial>,
    builder: B,
    builder_data: D,
    length: usize,
    height: Size,
    key: DiffKey,
}

/// Compared on everything **but** the builder, as [`VirtualScrollView`] is: two closures are never
/// equal, so deriving this would re-render the tree on every pass.
impl<D: PartialEq, B: Fn(VirtualItem, &D) -> Element> PartialEq for Tree<D, B> {
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
            && self.builder_data == other.builder_data
            && self.length == other.length
            && self.height == other.height
    }
}

impl<B: Fn(VirtualItem, &()) -> Element> Tree<(), B> {
    /// A tree whose rows are built by index.
    pub fn new(builder: B) -> Self {
        Self {
            theme: None,
            builder,
            builder_data: (),
            length: 0,
            height: Size::fill(),
            key: DiffKey::None,
        }
    }
}

impl<D, B: Fn(VirtualItem, &D) -> Element> Tree<D, B> {
    /// A tree whose row builder is handed shared data — the [`VirtualScrollView`] contract, and the
    /// way to keep a snapshot out of the memoized closure.
    pub fn new_with_data(builder_data: D, builder: B) -> Self {
        Self {
            theme: None,
            builder,
            builder_data,
            length: 0,
            height: Size::fill(),
            key: DiffKey::None,
        }
    }

    /// How many rows are visible in total (expanded rows included).
    pub fn length(mut self, length: impl Into<usize>) -> Self {
        self.length = length.into();
        self
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
        self
    }

    pub fn theme(mut self, theme: TreeThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl<D: PartialEq, B: Fn(VirtualItem, &D) -> Element> KeyExt for Tree<D, B> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<D: Clone + PartialEq + 'static, B: Clone + Fn(VirtualItem, &D) -> Element + 'static> Component
    for Tree<D, B>
{
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, TreeThemePreference, "tree");
        let TreeTheme {
            background,
            color,
            indent,
            item_height,
            corner_radius,
            ..
        } = theme;
        use_provide_context(|| TreeConfig {
            indent,
            item_height,
        });

        let builder = self.builder.clone();
        rect()
            .width(Size::fill())
            .height(self.height.clone())
            .background(background)
            .color(color)
            .corner_radius(corner_radius)
            .child(
                VirtualScrollView::new_with_data(self.builder_data.clone(), builder)
                    .length(self.length)
                    .item_size(item_height)
                    // A tree scrolls on both axes — down its rows and across a long value — so a
                    // gesture commits to whichever dominates rather than drifting the other. Without
                    // it, scrolling down a deep tree slides the rows sideways as it goes.
                    .wheel_axis_lock(WHEEL_AXIS_LOCK),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
