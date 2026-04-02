#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const SHADER: &str = r#"
uniform float3 iResolution;
uniform float iTime;

half4 main(float2 fragCoord) {
    float2 uv = fragCoord / iResolution.xy;

    half r = 0.5 + 0.5 * sin(iTime + uv.x * 3.14);
    half g = 0.5 + 0.5 * sin(iTime + uv.y * 3.14);
    half b = 0.5 + 0.5 * sin(iTime);

    return half4(r, g, b, 1.0);
}
"#;

fn app() -> impl IntoElement {
    let now = std::time::Instant::now();

    rect()
        .expanded()
        .background_shader(ShaderFill::new(SHADER, move |effect, bounds| {
            effect.make_shader(
                skia_safe::Data::new_copy(
                    &[
                        bounds.width().to_le_bytes(),
                        bounds.height().to_le_bytes(),
                        0.0f32.to_le_bytes(),
                        now.elapsed().as_secs_f32().to_le_bytes(),
                    ]
                    .concat(),
                ),
                &[],
                None,
            )
        }))
}
