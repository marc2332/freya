use freya_core::prelude::*;
use torin::{
    content::Content,
    prelude::{
        Alignment,
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
/// ```
#[derive(Default, Clone, PartialEq)]
pub struct Menu {
    children: Vec<Element>,
    on_close: Option<EventHandler<()>>,
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    pub fn on_close<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<()>>,
    {
        self.on_close = Some(f.into());
        self
    }
}

impl RenderOwned for Menu {
    fn render(self) -> impl IntoElement {
        // Provide the menus ID generator
        use_provide_context(|| State::create(ROOT_MENU.0));
        // Provide the menus stack
        use_provide_context::<State<Vec<MenuId>>>(|| State::create(vec![ROOT_MENU]));
        // Provide this the ROOT Menu ID
        use_provide_context(|| ROOT_MENU);

        rect()
            .corner_radius(8.0)
            .on_press(move |ev: Event<PressEventData>| {
                ev.stop_propagation();
            })
            .on_global_mouse_up(move |_| {
                if let Some(on_close) = &self.on_close {
                    on_close.call(());
                }
            })
            .child(MenuContainer::new().children(self.children))
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
    children: Vec<Element>,
    pub(crate) theme: Option<MenuContainerThemePartial>,
}

impl MenuContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }
}

impl RenderOwned for MenuContainer {
    fn render(self) -> impl IntoElement {
        let focus = use_focus();
        let theme = get_theme!(self.theme, menu_container);

        use_provide_context(move || focus.a11y_id());

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_member_of(focus.a11y_id())
            .position(Position::new_absolute())
            .shadow((0.0, 4.0, 10.0, 0., theme.shadow))
            .background(theme.background)
            .corner_radius(theme.corner_radius)
            .padding(theme.padding)
            .border(Border::new().width(1.).fill(theme.border_fill))
            .content(Content::fit())
            .children(self.children)
    }
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
#[derive(Default, Clone, PartialEq)]
pub struct MenuItem {
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_pointer_enter: Option<EventHandler<Event<PointerEventData>>>,
    pub(crate) theme: Option<MenuItemThemePartial>,
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
}

impl ChildrenExt for MenuItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl RenderOwned for MenuItem {
    fn render(self) -> impl IntoElement {
        let theme = get_theme!(self.theme, menu_item);
        let mut hovering = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let menu_group = use_consume::<AccessibilityId>();

        let background = if focus_status() == FocusStatus::Keyboard || *hovering.read() {
            theme.hover_background
        } else {
            Color::TRANSPARENT
        };

        let on_pointer_enter = move |e| {
            hovering.set(true);
            if let Some(on_pointer_enter) = &self.on_pointer_enter {
                on_pointer_enter.call(e);
            }
        };

        let on_pointer_leave = move |_| {
            hovering.set(false);
        };

        let on_press = move |e: Event<PressEventData>| {
            e.stop_propagation();
            e.prevent_default();
            focus.request_focus();
            if let Some(on_press) = &self.on_press {
                on_press.call(e);
            }
        };

        rect()
            .a11y_role(AccessibilityRole::Button)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(true)
            .a11y_member_of(menu_group)
            .min_width(Size::px(105.))
            .width(Size::fill_minimum())
            .padding((4.0, 10.0))
            .corner_radius(theme.corner_radius)
            .background(background)
            .color(theme.color)
            .text_align(TextAlign::Start)
            .main_align(Alignment::Center)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .on_press(on_press)
            .children(self.children)
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
    on_press: Option<EventHandler<()>>,
}

impl MenuButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    pub fn on_press<F>(mut self, f: F) -> Self
    where
        F: Into<EventHandler<()>>,
    {
        self.on_press = Some(f.into());
        self
    }
}

impl RenderOwned for MenuButton {
    fn render(self) -> impl IntoElement {
        let mut menus = use_consume::<State<Vec<MenuId>>>();
        let parent_menu_id = use_consume::<MenuId>();

        MenuItem::new()
            .on_pointer_enter(move |_| close_menus_until(&mut menus, parent_menu_id))
            .on_press(move |_| {
                if let Some(on_press) = &self.on_press {
                    on_press.call(());
                }
            })
            .children(self.children)
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

impl RenderOwned for SubMenu {
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

        let onmouseenter = move |_| {
            close_menus_until(&mut menus, parent_menu_id);
            push_menu(&mut menus, submenu_id);
        };

        let onpress = move |_| {
            close_menus_until(&mut menus, parent_menu_id);
            push_menu(&mut menus, submenu_id);
        };

        MenuItem::new()
            .on_pointer_enter(onmouseenter)
            .on_press(onpress)
            .child(rect().horizontal().maybe_child(self.label.clone()))
            .maybe_child(show_submenu.then(|| {
                rect()
                    .position(Position::new_absolute().top(-8.).right(-16.))
                    .width(Size::px(0.))
                    .height(Size::px(0.))
                    .child(
                        rect()
                            .width(Size::window_percent(100.))
                            .child(MenuContainer::new().children(self.items)),
                    )
            }))
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
