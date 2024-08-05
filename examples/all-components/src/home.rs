use dioxus_router::prelude::{Outlet, Routable};
use freya::prelude::*;
use strum::IntoEnumIterator;

use crate::DsGui;

use super::Route;

#[allow(non_snake_case)]
pub fn Home() -> Element {
    rsx!(
        rect {
            main_align: "start",
            padding: "50",
            background: "rgb(200, 200, 200)",
            corner_radius: "10",
            label{
                font_size: "28",
                "Welcome to Freya components preview"
            }
            rect{
                height: "10"
            }
            label{
                "Here you can:"
            }
            rect{
                height: "10"
            }
            label{
                "- View all components"
            }
            rect{
                height: "10"
            }
            label{
                "- Play with theming"
            }
        }
    )
}

#[allow(non_snake_case)]
pub fn AppSidebar() -> Element {
    let mut search = use_signal(|| String::default());

    rsx!(
        NativeRouter {
            Sidebar {
                sidebar: rsx!(
                    Input {
                        value: search(),
                        placeholder: "Search...",
                        onchange: move |e| {
                             search.set(e)
                        }
                    }
                    for route in Route::iter().filter(|r| r != &Route::PageNotFound {  } ) {
                        {
                            let (code, entry_type) = route_entry(route);
                            if search().is_empty()
                                || code.map(|c|
                                    c.to_lowercase().contains(search().to_lowercase().as_str())
                                ).unwrap_or_default() {
                                rsx!(
                                    SideBarEntry {
                                        route: route,
                                        code: code,
                                        entry_type: entry_type
                                    }
                                )
                            }
                            else {
                                rsx!()
                            }
                        }
                    }
                ),
                Body {
                    DsGui {
                        rect {
                            main_align: "center",
                            cross_align: "start",
                            width: "100%",
                            height: "100%",
                            padding: "20",
                            Outlet::<Route> {  }
                        }
                    }
                }
            }
        }
    )
}

fn route_entry(route: Route) -> (Option<&'static str>, &'static str) {
    match route {
        Route::Home => (None, "Welcome"),
        Route::DsImage => (Some("image"), "element"),
        Route::DsLabel => (Some("label"), "element"),
        Route::DsParagraph => (Some("paragraph"), "element"),
        Route::DsRect => (Some("rect"), "element"),
        Route::DsSvg => (Some("svg"), "element"),
        Route::DsText => (Some("text"), "element"),
        Route::DsAccordion => (Some("Accordion"), "component"),
        Route::DsActivableRoute => (Some("ActivableRoute"), "component"),
        Route::DsArrowIcon => (Some("ArrowIcon"), "component"),
        Route::DsButton => (Some("Button"), "component"),
        // Route::DsCanvas => (Some("Canvas"), "component"),
        Route::DsCheckbox => (Some("Checkbox"), "component"),
        Route::DsCrossIcon => (Some("CrossIcon"), "component"),
        // Route::DsCursorArea => (Some("CursorArea"), "component"),
        Route::DsDragProvider => (Some("DragProvider"), "component"),
        Route::DsDropdown => (Some("Dropdown"), "component"),
        // Route::DsGestureArea => (Some("GestureArea"), "component"),
        // Route::DsGraph => (Some("Graph"), "component"),
        Route::DsInput => (Some("Input"), "component"),
        // Route::DsKeyboardNavigator => (Some("KeyboardNavigator"), "component"),
        Route::DsLink => (Some("Link"), "component"),
        Route::DsLoader => (Some("Loader"), "component"),
        Route::DsMenu => (Some("Menu"), "component"),
        // Route::DsMenuButton => (Some("MenuButton"), "component"),
        // Route::DsNetworkImage => (Some("NetworkImage"), "component"),
        Route::DsPopup => (Some("Popup"), "component"),
        Route::DsProgressBar => (Some("ProgressBar"), "component"),
        Route::DsRadio => (Some("Radio"), "component"),
        // Route::DsScrollView => (Some("ScrollView"), "component"),
        // Route::DsScrollBar => (Some("ScrollBar"), "component"),
        // Route::DsScrollThumb => (Some("ScrollThumb"), "component"),
        // Route::DsSideBar => (Some("SideBar"), "component"),
        Route::DsSlider => (Some("Slider"), "component"),
        Route::DsSnackBar => (Some("SnackBar"), "component"),
        // Route::DsSubMenu => (Some("SubMenu"), "component"),
        Route::DsSwitch => (Some("Switch"), "component"),
        Route::DsTable => (Some("Table"), "component"),
        Route::DsTickIcon => (Some("TickIcon"), "component"),
        // Route::DsTile => (Some("Tile"), "component"),
        Route::DsTooltip => (Some("Tooltip"), "component"),
        // Route::DsVirtualScrollView => (Some("VirtualScrollView"), "component"),
        _ => unimplemented!(),
    }
}

#[component]
fn SideBarEntry(route: Route, code: Option<&'static str>, entry_type: &'static str) -> Element {
    rsx!(
        Link {
            to: route,
            ActivableRoute {
                route: route,
                exact: true,
                SidebarItem {
                    if let Some(code) = code {
                        label {
                            font_style: "mono",
                            {code}
                        }
                        label {
                            font_size: "11",
                            {entry_type}
                        }
                    }
                    else {
                        label {
                            {entry_type}
                        }
                    }
                },
            }
        }
    )
}
