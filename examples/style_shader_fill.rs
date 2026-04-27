#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    engine::prelude::{
        Data,
        RuntimeEffect,
    },
    prelude::*,
};

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
    let (now, effect) = use_hook(|| {
        // Create the effect once and reuse in the shader render function
        (
            std::time::Instant::now(),
            RuntimeEffect::make_for_shader(SHADER, None).expect("shader compilation failed"),
        )
    });

    use_future(move || {
        async move {
            let mut ticket = RenderingTicker::get();
            loop {
                // Continuously render the app so that the shader updates on live
                ticket.tick().await;
                Platform::get().send(UserEvent::RequestRedraw);
            }
        }
    });

    rect()
        .expanded()
        .background_shader(ShaderFill::new(SHADER, effect, move |effect, bounds| {
            effect.make_shader(
                Data::new_copy(
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
