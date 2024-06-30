#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ds;
mod home;
mod not_found;

use dioxus_router::prelude::{Routable, Router};
use ds::*;
use freya::prelude::*;
use home::*;
use not_found::*;
use strum::EnumIter;

fn main() {
    launch_with_props(app, "Freya Components Preview", (800.0, 600.0));
}

fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, Copy, PartialEq, EnumIter)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[nest("/elements")]
            #[route("/image")]
            DsImage,
            #[route("/label")]
            DsLabel,
            #[route("/paragraph")]
            DsParagraph,
            #[route("/rect")]
            DsRect,
            #[route("/svg")]
            DsSvg,
            #[route("/text")]
            DsText,
        #[end_nest]
        #[nest("/components")]
            #[route("/arrow-icon")]
            DsArrowIcon,
            #[route("/cross-icon")]
            DsCrossIcon,
            #[route("/tick-icon")]
            DsTickIcon,
            #[route("/accordion")]
            DsAccordion,
            #[route("/activable-route")]
            DsActivableRoute,
            #[route("/button")]
            DsButton,
            // #[route("/canvas")]
            // DsCanvas,
            #[route("/checkbox")]
            DsCheckbox,
            // #[route("/cursor-area")]
            // DsCursorArea,
            #[route("/drag-provider")]
            DsDragProvider,
            #[route("/dropdown")]
            DsDropdown,
            // #[route("/gesture-area")]
            // DsGestureArea,
            // #[route("/graph")]
            // DsGraph,
            #[route("/input")]
            DsInput,
            // #[route("/keyboard-navigator")]
            // DsKeyboardNavigator,
            #[route("/link")]
            DsLink,
            #[route("/loader")]
            DsLoader,
            #[route("/menu")]
            DsMenu,
            // #[route("/network-image")]
            // DsNetworkImage,
            #[route("/popup")]
            DsPopup,
            #[route("/progress-bar")]
            DsProgressBar,
            #[route("/radio")]
            DsRadio,
            // #[route("/scroll-view")]
            // DsScrollView,
            // #[route("/scroll-bar")]
            // DsScrollBar,
            // #[route("/scroll-thumb")]
            // DsScrollThumb,
            // #[route("/side-bar")]
            // DsSideBar,
            #[route("/slider")]
            DsSlider,
            #[route("/snack-bar")]
            DsSnackBar,
            #[route("/switch")]
            DsSwitch,
            #[route("/table")]
            DsTable,
            // #[route("/tile")]
            // DsTile,
            #[route("/tooltip")]
            DsTooltip,
            // #[route("/virtual-scorll-view")]
            // DsVirtualScrollView,
        #[end_nest]
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}
