#[cfg(target_os = "android")]
use freya::prelude::{
    LaunchConfig,
    WindowConfig,
    launch,
};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
mod app;

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

    let event_loop = EventLoop::<NativeEvent>::with_user_event()
        .with_android_app(droid_app)
        .build()
        .expect("Failed to build event loop");

    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app::app).with_size(500., 450.))
            .with_event_loop(event_loop),
    )
}
