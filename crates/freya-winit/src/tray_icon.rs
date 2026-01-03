use tray_icon::{
    TrayIconEvent,
    menu::MenuEvent,
};

#[derive(Clone, Debug)]
pub enum TrayEvent {
    Icon(TrayIconEvent),
    Menu(MenuEvent),
}
