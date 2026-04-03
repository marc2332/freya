use freya_core::prelude::*;
use torin::{
    content::Content,
    gaps::Gaps,
    prelude::{
        Alignment,
        Area,
        Position,
    },
    size::Size,
};

use crate::{
    get_theme,
    theming::component_themes::{
        MenuContainerThemePartial,
        MenuItemThemePartial,
    },
};

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
/// # }, "./images/gallery_menu.png").render();
/// ```
///
/// # Preview
/// ![Menu Preview][menu]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("menu", "images/gallery_menu.png"),
)]
#[derive(Default, Clone, PartialEq)]
pub struct Menu {
    children: Vec<Element>,
    on_close: Option<EventHandler<()>>,
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

    pub fn on_close<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<()>>,
    {
        self.on_close = Some(f.into());
        self
    }
}

impl ComponentOwned for Menu {
    fn render(self) -> impl IntoElement {
        // Provide the menus ID generator
        use_provide_context(|| State::create(ROOT_MENU.0));
        // Provide the menus stack
        use_provide_context::<State<Vec<MenuId>>>(|| State::create(vec![ROOT_MENU]));
        // Provide this the ROOT Menu ID
        use_provide_context(|| ROOT_MENU);

        rect()
            .layer(Layer::Overlay)
            .corner_radius(8.0)
            .on_press(move |ev: Event<PressEventData>| {
                ev.stop_propagation();
            })
            .on_global_pointer_press(move |_: Event<PointerEventData>| {
                if let Some(on_close) = &self.on_close {
                    on_close.call(());
                }
            })
            .child(MenuContainer::new().children(self.children))
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
}

impl ComponentOwned for MenuContainer {
    fn render(self) -> impl IntoElement {
        let focus = use_focus();
        let theme = get_theme!(self.theme, menu_container);
        let mut measured = use_state(|| None::<(Area, f32, f32)>);

        use_provide_context(move || MenuGroup {
            group_id: focus.a11y_id(),
        });

        let (offset_x, offset_y, opacity) = match *measured.read() {
            None => (0.0, 0.0, 0.0),
            Some((area, win_w, win_h)) => (
                overflow_offset(area.origin.x, area.size.width, win_w),
                overflow_offset(area.origin.y, area.size.height, win_h),
                1.0,
            ),
        };

        rect()
            .layer(Layer::Overlay)
            .content(Content::fit())
            .opacity(opacity)
            .offset_x(offset_x)
            .offset_y(offset_y)
            .on_sized(move |e: Event<SizedEventData>| {
                if measured.peek().is_none() {
                    let window = Platform::get().root_size.peek();
                    measured.set(Some((e.area, window.width, window.height)));
                }
            })
            .child(
                rect()
                    .a11y_id(focus.a11y_id())
                    .a11y_member_of(focus.a11y_id())
                    .a11y_focusable(true)
                    .a11y_role(AccessibilityRole::Menu)
                    .shadow((0.0, 4.0, 10.0, 0., theme.shadow))
                    .background(theme.background)
                    .corner_radius(theme.corner_radius)
                    .padding(theme.padding)
                    .border(Border::new().width(1.).fill(theme.border_fill))
                    .content(Content::fit())
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
        let theme = get_theme!(self.theme, menu_item);
        let mut hovering = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let MenuGroup { group_id } = use_consume::<MenuGroup>();

        let background = if self.selected {
            theme.select_background
        } else if hovering() {
            theme.hover_background
        } else {
            theme.background
        };

        let border = if focus_status() == FocusStatus::Keyboard {
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
                focus.request_focus();
            }
        };

        rect()
            .a11y_role(AccessibilityRole::MenuItem)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(true)
            .a11y_member_of(group_id)
            .min_width(Size::px(105.))
            .width(Size::fill_minimum())
            .content(Content::fit())
            .padding(self.padding)
            .corner_radius(theme.corner_radius)
            .background(background)
            .border(border)
            .color(theme.color)
            .text_align(TextAlign::Start)
            .main_align(Alignment::Center)
            .overflow(Overflow::Clip)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .on_press(on_press)
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
#[derive(Default, Clone, PartialEq)]
pub struct MenuButton {
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
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
}

impl ComponentOwned for MenuButton {
    fn render(self) -> impl IntoElement {
        let mut menus = use_consume::<State<Vec<MenuId>>>();
        let parent_menu_id = use_consume::<MenuId>();

        MenuItem::new()
            .on_pointer_enter(move |_| close_menus_until(&mut menus, parent_menu_id))
            .map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
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
                        rect()
                            .width(Size::window_percent(100.))
                            .child(MenuContainer::new().children(self.items)),
                    )
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Returns a negative offset to shift an element back within the window boundary,
/// or `0.0` if it already fits.
fn overflow_offset(origin: f32, size: f32, window: f32) -> f32 {
    let overflow = origin + size - window;
    if overflow > 0.0 {
        -overflow.min(origin)
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
