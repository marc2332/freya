use freya::prelude::*;

#[cfg(target_os="android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os="android")]
#[no_mangle]
fn android_main(droid_app: AndroidApp) {

    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug));
    log::info!("android_main started!");
    let event_loop_hook : EventLoopBuilderHook = Box::new(move |event_loop_builder| {
        event_loop_builder.with_android_app(droid_app);
    });
    launch_cfg(
        app,
        LaunchConfig::<()> {
            window_config: WindowConfig {
                size: (600.0, 600.0),
                decorations: true,
                transparent: false,
                title: "Freya",
                event_loop_builder_hook: Some(event_loop_hook),
                ..Default::default()
            },
            ..Default::default()
        }
    )
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();
    launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label {
                font_size: "75",
                font_weight: "bold",
                "{count}"
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                //onclick: move |_| count += 1, 'onclick' does not work on Android
                onpress: move |_| count += 1,
                label { "Increase" }
            }
            Button {
                //onclick: move |_| count -= 1,
                onpress: move |_| count -= 1,
                label { "Decrease" }
            }
        }
    )
}
