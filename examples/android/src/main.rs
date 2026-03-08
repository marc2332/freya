use freya::prelude::{
    LaunchConfig,
    WindowConfig,
    launch,
    *,
};
#[cfg(target_os = "android")]
use {
    jni::{
        JNIEnv,
        JavaVM,
    },
    winit::platform::android::activity::AndroidApp,
};

fn app() -> impl IntoElement {
    let mut count = use_state(|| 4);

    let counter = rect()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .color((255, 255, 255))
        .background((15, 163, 242))
        .font_weight(FontWeight::BOLD)
        .font_size(75.)
        .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
        .child(count.read().to_string());

    let actions = rect()
        .horizontal()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() += 1;
                })
                .child("Increase"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() -= 1;
                })
                .child("Decrease"),
        );

    rect().child(counter).child(actions)
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::init();
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(500., 450.)
                .with_resizable(false),
        ),
    )
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(droid_app: AndroidApp) {
    use freya_winit::renderer::NativeEvent;
    use winit::{
        event_loop::EventLoop,
        platform::android::EventLoopBuilderExtAndroid,
    };

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    let mut event_loop_builder = EventLoop::<NativeEvent>::with_user_event();
    event_loop_builder.with_android_app(droid_app);

    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app).with_size(500., 450.))
            .with_event_loop_builder(event_loop_builder),
    )
}
