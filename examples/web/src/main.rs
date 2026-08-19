mod app;

#[cfg(target_os = "emscripten")]
static NOTO_SANS: &[u8] = include_bytes!("../../../crates/freya-edit/tests/NotoSans-Regular.ttf");

fn main() {
    #[cfg(target_os = "emscripten")]
    freya_web::launch_cfg(freya_web::WebConfig::new(app::app).with_font("Noto Sans", NOTO_SANS));
}
