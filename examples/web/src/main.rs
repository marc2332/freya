#![cfg_attr(not(target_os = "emscripten"), allow(dead_code))]

mod app;
mod showcases;

#[cfg(target_os = "emscripten")]
const NOTO_SANS: &[u8] = include_bytes!("../../../crates/freya-edit/tests/NotoSans-Regular.ttf");

fn main() {
    #[cfg(target_os = "emscripten")]
    freya::web::launch_cfg(freya::web::WebConfig::new(app::app).with_font("Noto Sans", NOTO_SANS));
}
