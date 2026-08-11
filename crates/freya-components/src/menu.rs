use freya_core::prelude::*;
use torin::{
    content::Content,
    gaps::Gaps,
    prelude::{
        Alignment,
        Area,
        Position,
        Size2D,
    },
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
};

define_theme! {
    for = MenuContainer; theme_field = theme;
    for = Menu; theme_field = theme;
    for = SubMenu; theme_field = theme;

    %[component]
    pub MenuContainer {
        %[fields]
        background: Color,
        padding: Gaps,
        shadow: Color,
        border_fill: Color,
        corner_radius: CornerRadius,
    }
}

define_theme! {
    for = MenuItem; theme_field = theme;
    for = MenuButton; theme_field = theme;

    %[component]
    pub MenuItem {
        %[fields]
        background: Color,
        hover_background: Color,
        select_background: Color,
        border_fill: Color,
        select_border_fill: Color,
        corner_radius: CornerRadius,
        color: Color,
    }
}

/// Floating menu container.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut show_menu = use_state(|| false);
///
///     rect()
///         .child(
///             Button::new()
///                 .on_press(move |_| show_menu.toggle())
///                 .child("Open Menu"),
///         )
///         .maybe_child(show_menu().then(|| {
///             Menu::new()
///                 .on_close(move |_| show_menu.set(false))
///                 .child(MenuButton::new().child("Open"))
///                 .child(MenuButton::new().child("Save"))
///                 .child(
///                     SubMenu::new()
///                         .label("Export")
///                         .child(MenuButton::new().child("PDF")),
///                 )
///         }))
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   let mut show_menu = use_state(|| true);
/// #   rect().center().expanded().child(
/// #       rect()
/// #           .child(
/// #               Button::new()
/// #                   .on_press(move |_| show_menu.toggle())
/// #                   .child("Open Menu"),
/// #           )
/// #           .maybe_child(show_menu().then(|| {
/// #               Menu::new()
/// #                   .on_close(move |_| show_menu.set(false))
/// #                   .child(MenuButton::new().child("Open"))
/// #                   .child(MenuButton::new().child("Save"))
/// #           }))
/// #   )
/// # }, "./images/gallery_menu.png").with_hook(|t| { t.poll(std::time::Duration::from_millis(1), std::time::Duration::from_millis(100)); }).render();
/// ```
///
/// # Preview
/// ![Menu Preview][menu]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("menu", "images/gallery_menu.png"),
)]
#[derive(Default, Clone, PartialEq)]
pub struct Menu {
    pub(crate) theme: Option<MenuContainerThemePartial>,
    children: Vec<Element>,
    on_close: Option<EventHandler<()>>,
    on_escape: Option<EventHandler<()>>,
    min_width: Option<Size>,
    min_height: Option<Size>,
    key: DiffKey,
}

impl ChildrenExt for Menu {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Menu {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    /// Called when Escape closes the menu, falls back to [`Menu::on_close`].
    pub fn on_escape(mut self, f: impl Into<EventHandler<()>>) -> Self {
        self.on_escape = Some(f.into());
        self
    }

    pub fn on_close<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<()>>,
    {
        self.on_close = Some(f.into());
        self
    }

    pub fn theme(mut self, theme: MenuContainerThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    /// A minimum width for the menu's container, so short items don't collapse / wrap.
    pub fn min_width(mut self, min_width: impl Into<Size>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }

    /// A minimum height for the menu's container.
    pub fn min_height(mut self, min_height: impl Into<Size>) -> Self {
        self.min_height = Some(min_height.into());
        self
    }
}

impl ComponentOwned for Menu {
    fn render(self) -> impl IntoElement {
        // Provide the menus ID generator
        use_provide_context(|| State::create(ROOT_MENU.0));
        // Provide the menus stack
        let mut menus =
            use_provide_context::<State<Vec<MenuId>>>(|| State::create(vec![ROOT_MENU]));
        // Provide this the ROOT Menu ID
        use_provide_context(|| ROOT_MENU);

        // The menu's own laid-out area, so an outside-click can be told apart from a click on the
        // menu's own content (e.g. a focusable `Input` inside it — whose press `stop_propagation`
        // can't suppress this *global* handler, since the global press is a separate event).
        let mut menu_area = use_state(Area::default);
        // Outside-press closing arms on the first pointer-*down* after mount: the click that
        // *opened* the menu already went down before the menu existed, so its release (a global
        // press event) must never count as the closing click. A real outside click goes
        // down (arms) then presses (closes), still one click.
        let mut armed = use_state(|| false);

        let on_escape = self.on_escape.clone().or(self.on_close.clone());
        let on_global_key_down = move |e: Event<KeyboardEventData>| {
            if e.key == Key::Named(NamedKey::Escape) {
                if menus.read().len() > 1 {
                    menus.write().pop();
                    // Consume the Escape: cancels the remaining global key events so
                    // deeper listeners don't also act on the same press.
                    e.prevent_default();
                } else if let Some(on_escape) = &on_escape {
                    on_escape.call(());
                    e.prevent_default();
                }
            }
        };

        rect()
            .layer(Layer::Overlay)
            .corner_radius(8.0)
            .on_sized(move |e: Event<SizedEventData>| menu_area.set(e.area))
            .on_press(move |ev: Event<PressEventData>| {
                ev.stop_propagation();
            })
            .on_global_pointer_down(move |_| armed.set_if_modified(true))
            .on_global_pointer_press(move |e: Event<PointerEventData>| {
                // Close only when armed (see above) and the press landed outside the
                // menu's own bounds.
                if !armed() {
                    return;
                }
                let p = e.data().global_location();
                let a = *menu_area.read();
                let (px, py) = (p.x as f32, p.y as f32);
                let outside = px < a.origin.x
                    || px > a.origin.x + a.size.width
                    || py < a.origin.y
                    || py > a.origin.y + a.size.height;
                if outside && let Some(on_close) = &self.on_close {
                    on_close.call(());
                }
            })
            .on_global_key_down(on_global_key_down)
            .child(
                MenuContainer::new()
                    .map(self.theme, |el, theme| el.theme(theme))
                    .map(self.min_width, |el, w| el.min_width(w))
                    .map(self.min_height, |el, h| el.min_height(h))
                    .children(self.children),
            )
    }
    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Container for menu items with proper spacing and layout.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     MenuContainer::new()
///         .child(MenuItem::new().child("Item 1"))
///         .child(MenuItem::new().child("Item 2"))
/// }
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct MenuContainer {
    pub(crate) theme: Option<MenuContainerThemePartial>,
    children: Vec<Element>,
    min_width: Option<Size>,
    min_height: Option<Size>,
    key: DiffKey,
}

impl KeyExt for MenuContainer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for MenuContainer {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl MenuContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn theme(mut self, theme: MenuContainerThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    /// A minimum width for the container box.
    pub fn min_width(mut self, min_width: impl Into<Size>) -> Self {
        self.min_width = Some(min_width.into());
        self
    }

    /// A minimum height for the container box.
    pub fn min_height(mut self, min_height: impl Into<Size>) -> Self {
        self.min_height = Some(min_height.into());
        self
    }
}

impl ComponentOwned for MenuContainer {
    fn render(self) -> impl IntoElement {
        let a11y_id = use_a11y();
        let theme = get_theme!(self.theme, MenuContainerThemePreference, "menu_container");
        let mut measured = use_state(|| None::<(Area, Size2D)>);

        use_provide_context(move || MenuGroup { group_id: a11y_id });

        // Inside an `Attached` overlay the position arrives already window-clamped
        // (`AttachedHosted`), and a second self-measured correction would lag a frame
        // behind it and paint a visible jump, so skip it. The self-correction remains for
        // directly-positioned hosts (the `ContextMenu` overlay at the cursor), where the
        // first measurement is already at the final position.
        let hosted = try_consume_context::<crate::attached::AttachedHosted>().is_some();
        let (offset_x, offset_y, opacity) = if hosted {
            (0.0, 0.0, 1.0)
        } else {
            match measured() {
                None => (0.0, 0.0, 0.0),
                Some((area, root_size)) => (
                    overflow_offset(area.origin.x, area.size.width, root_size.width),
                    overflow_offset(area.origin.y, area.size.height, root_size.height),
                    1.0,
                ),
            }
        };

        rect()
            .layer(Layer::Overlay)
            .content(Content::fit())
            .opacity(opacity)
            .offset_x(offset_x)
            .offset_y(offset_y)
            .on_sized(move |e: Event<SizedEventData>| {
                // Track every re-measure, not just the first: inside an `Attached` overlay
                // the first `Sized` fires *before* the attached positioning applies, so a
                // frozen first measurement locks the window-overflow correction to a
                // pre-position area (a first-open menu lands visibly displaced). The
                // offsets shift this rect's *children*, never its own area, so updating
                // here cannot loop.
                let root_size = *Platform::get().root_size.peek();
                measured.set_if_modified(Some((e.area, root_size)));
            })
            .child(
                rect()
                    .a11y_id(a11y_id)
                    .a11y_member_of(a11y_id)
                    .a11y_focusable(true)
                    .a11y_role(AccessibilityRole::Menu)
                    .shadow((0.0, 4.0, 10.0, 0., theme.shadow))
                    .background(theme.background)
                    .corner_radius(theme.corner_radius)
                    .padding(theme.padding)
                    .border(Border::new().width(1.).fill(theme.border_fill))
                    .content(Content::fit())
                    .map(self.min_width, |el, w| el.min_width(w))
                    .map(self.min_height, |el, h| el.min_height(h))
                    .children(self.children),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone)]
pub struct MenuGroup {
    pub group_id: AccessibilityId,
}

/// A clickable menu item with hover and focus states.
///
/// This is the base component used by MenuButton and SubMenu.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     MenuItem::new()
///         .on_press(|_| println!("Clicked!"))
///         .child("Open File")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub(crate) theme: Option<MenuItemThemePartial>,
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_pointer_enter: Option<EventHandler<Event<PointerEventData>>>,
    selected: bool,
    enabled: bool,
    padding: Gaps,
    key: DiffKey,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            on_press: None,
            on_pointer_enter: None,
            selected: false,
            enabled: true,
            padding: (6.0, 12.0).into(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for MenuItem {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl MenuItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<Event<PressEventData>>>,
    {
        self.on_press = Some(f.into());
        self
    }

    pub fn on_pointer_enter<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<Event<PointerEventData>>>,
    {
        self.on_pointer_enter = Some(f.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Whether the item is interactive. A disabled item renders dimmed, never highlights on
    /// hover, drops its press handler, and is not focusable.
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    /// Set the padding for this menu item.
    pub fn padding(mut self, padding: impl Into<Gaps>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Get the current padding.
    pub fn get_padding(&self) -> Gaps {
        self.padding
    }

    /// Get the theme override for this component.
    pub fn get_theme(&self) -> Option<&MenuItemThemePartial> {
        self.theme.as_ref()
    }

    /// Set a theme override for this component.
    pub fn theme(mut self, theme: MenuItemThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ChildrenExt for MenuItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl ComponentOwned for MenuItem {
    fn render(self) -> impl IntoElement {
        let theme = get_theme!(self.theme, MenuItemThemePreference, "menu_item");
        let mut hovering = use_state(|| false);
        let a11y_id = use_a11y();
        let focus = use_focus(a11y_id);
        let MenuGroup { group_id } = use_consume::<MenuGroup>();
        let enabled = self.enabled;

        let background = if self.selected {
            theme.select_background
        } else if hovering() && enabled {
            theme.hover_background
        } else {
            theme.background
        };

        let border = if enabled && focus() == Focus::Keyboard {
            Border::new()
                .fill(theme.select_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme.border_fill)
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        let on_pointer_enter = move |e: Event<PointerEventData>| {
            hovering.set(true);
            if let Some(on_pointer_enter) = &self.on_pointer_enter {
                on_pointer_enter.call(e);
            }
        };

        let on_pointer_leave = move |_| {
            hovering.set(false);
        };

        let on_press = move |e: Event<PressEventData>| {
            let prevent_default = e.get_prevent_default();
            if let Some(on_press) = &self.on_press {
                on_press.call(e);
            }
            if *prevent_default.borrow() {
                a11y_id.request_focus();
            }
        };

        rect()
            .a11y_role(AccessibilityRole::MenuItem)
            .a11y_id(a11y_id)
            .a11y_focusable(enabled)
            .a11y_member_of(group_id)
            .min_width(Size::px(105.))
            .width(Size::fill_minimum())
            .content(Content::fit())
            .padding(self.padding)
            .corner_radius(theme.corner_radius)
            .background(background)
            .border(border)
            .color(theme.color)
            // Fade the whole row toward the menu background when disabled. Opacity (not a
            // colour tint) so it reads as muted on light and dark themes alike.
            .maybe(!enabled, |el| el.opacity(0.5))
            .text_align(TextAlign::Start)
            .main_align(Alignment::Center)
            .overflow(Overflow::Clip)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .maybe(enabled, |el| el.on_press(on_press))
            .children(self.children)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Like a button, but for Menus.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     MenuButton::new()
///         .on_press(|_| println!("Clicked!"))
///         .child("Item")
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct MenuButton {
    pub(crate) theme: Option<MenuItemThemePartial>,
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    selected: bool,
    enabled: bool,
    key: DiffKey,
}

impl Default for MenuButton {
    fn default() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            on_press: None,
            selected: false,
            enabled: true,
            key: DiffKey::None,
        }
    }
}

impl ChildrenExt for MenuButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for MenuButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl MenuButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    /// Draw this button as the selected one, in [`MenuItem`]'s selected background.
    ///
    /// For a menu that is a list of choices with a current one, or one a surface is driving from
    /// the keyboard: the highlighted row reads the same whether the pointer is on it or the arrow
    /// keys are.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Whether the button is interactive. A disabled button renders dimmed and does nothing on
    /// press.
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    /// Set a theme override for the inner [`MenuItem`].
    pub fn theme(mut self, theme: MenuItemThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ComponentOwned for MenuButton {
    fn render(self) -> impl IntoElement {
        let mut menus = use_consume::<State<Vec<MenuId>>>();
        let parent_menu_id = use_consume::<MenuId>();

        MenuItem::new()
            .map(self.theme, |el, theme| el.theme(theme))
            .selected(self.selected)
            .enabled(self.enabled)
            .on_pointer_enter(move |_| close_menus_until(&mut menus, parent_menu_id))
            .map(self.on_press, |el, on_press| el.on_press(on_press))
            .children(self.children)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Create sub menus inside a Menu.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     SubMenu::new()
///         .label("Export")
///         .child(MenuButton::new().child("PDF"))
/// }
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct SubMenu {
    pub(crate) theme: Option<MenuContainerThemePartial>,
    label: Option<Element>,
    items: Vec<Element>,
    key: DiffKey,
}

impl KeyExt for SubMenu {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl SubMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.label = Some(label.into_element());
        self
    }

    /// Set a theme override for the inner [`MenuContainer`].
    pub fn theme(mut self, theme: MenuContainerThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ChildrenExt for SubMenu {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.items
    }
}

impl ComponentOwned for SubMenu {
    fn render(self) -> impl IntoElement {
        let parent_menu_id = use_consume::<MenuId>();
        let mut menus = use_consume::<State<Vec<MenuId>>>();
        let mut menus_ids_generator = use_consume::<State<usize>>();

        let submenu_id = use_hook(|| {
            *menus_ids_generator.write() += 1;
            let menu_id = MenuId(*menus_ids_generator.peek());
            provide_context(menu_id);
            menu_id
        });

        let show_submenu = menus.read().contains(&submenu_id);

        let on_pointer_enter = move |_| {
            close_menus_until(&mut menus, parent_menu_id);
            push_menu(&mut menus, submenu_id);
        };

        let on_press = move |_| {
            close_menus_until(&mut menus, parent_menu_id);
            push_menu(&mut menus, submenu_id);
        };

        MenuItem::new()
            .on_pointer_enter(on_pointer_enter)
            .on_press(on_press)
            .child(rect().horizontal().maybe_child(self.label.clone()))
            .maybe_child(show_submenu.then(|| {
                rect()
                    .position(Position::new_absolute().top(-8.).right(-10.))
                    .width(Size::px(0.))
                    .height(Size::px(0.))
                    .child(
                        rect().width(Size::window_percent(100.)).child(
                            MenuContainer::new()
                                .map(self.theme, |el, theme| el.theme(theme))
                                .children(self.items),
                        ),
                    )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Breathing room a corrected overlay keeps from the window edge, so it never sits flush
/// against the boundary. Shared by the overlay components ([`MenuContainer`], `TooltipContainer`).
pub(crate) const EDGE_MARGIN: f32 = 2.;

/// Returns the offset that shifts an element back within the window boundary (keeping
/// [`EDGE_MARGIN`] from it): positive when it hangs off the start edge, negative when it
/// overflows the end edge, or `0.0` if it already fits.
pub(crate) fn overflow_offset(origin: f32, size: f32, window: f32) -> f32 {
    if origin < EDGE_MARGIN {
        return EDGE_MARGIN - origin;
    }
    let overflow = origin + size - (window - EDGE_MARGIN);
    if overflow > 0.0 {
        -overflow.min(origin - EDGE_MARGIN)
    } else {
        0.0
    }
}

static ROOT_MENU: MenuId = MenuId(0);

#[derive(Clone, Copy, PartialEq, Eq)]
struct MenuId(usize);

fn close_menus_until(menus: &mut State<Vec<MenuId>>, until: MenuId) {
    menus.write().retain(|&id| id.0 <= until.0);
}

fn push_menu(menus: &mut State<Vec<MenuId>>, id: MenuId) {
    if !menus.read().contains(&id) {
        menus.write().push(id);
    }
}
