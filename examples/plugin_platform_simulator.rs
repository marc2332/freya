#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use freya_platform_simulator_plugin::PlatformSimulatorPlugin;

fn main() {
    launch(
        LaunchConfig::new()
            .with_plugin(PlatformSimulatorPlugin::new())
            .with_window(WindowConfig::new(app).with_title("Platform Simulator")),
    )
}

fn app() -> impl IntoElement {
    let mut name = use_state(String::new);
    let mut notifications = use_state(|| false);
    let mut volume = use_state(|| 50.);

    let platform = TargetPlatform::get();
    let native_platform = TargetPlatform::detect();
    let platforms = [
        ("Native", native_platform),
        ("Android", TargetPlatform::Android),
        ("Linux", TargetPlatform::Linux),
    ];
    let selected = platforms
        .iter()
        .find(|(_, target)| *target == platform)
        .map_or("Native", |(label, _)| *label);

    ScrollView::new().expanded().child(
        rect()
            .padding(16.)
            .spacing(12.)
            .child("Platform")
            .child(
                Select::new().selected_item(selected).children(
                    platforms
                        .into_iter()
                        .filter(|(label, target)| *label == "Native" || *target != native_platform)
                        .map(|(label, target)| {
                            MenuItem::new()
                                .selected(label == selected)
                                .on_press(move |_| TargetPlatform::set(target))
                                .child(label)
                        }),
                ),
            )
            .child(Input::new(name).placeholder("Name").width(Size::px(220.)))
            .child("Notifications")
            .child(Switch::new(notifications))
            .child("Volume")
            .child(
                Slider::new(move |value| volume.set(value))
                    .value(volume())
                    .size(Size::px(220.)),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        name.set(String::new());
                        notifications.set(false);
                        volume.set(50.);
                    })
                    .child("Reset"),
            )
            .children(
                [
                    (186, 216, 250),
                    (186, 232, 210),
                    (255, 224, 174),
                    (223, 204, 247),
                ]
                .map(|color| {
                    rect()
                        .width(Size::fill())
                        .height(Size::px(140.))
                        .corner_radius(8.)
                        .background(color)
                }),
            ),
    )
}
