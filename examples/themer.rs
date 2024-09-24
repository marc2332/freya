#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Themer", (400.0, 350.0));
}

fn app() -> Element {
    use_init_default_theme();
    let mut theme = use_theme();
    let mut r = use_signal::<f64>(|| 103. / 255. * 100.);
    let mut g = use_signal::<f64>(|| 80. / 255. * 100.);
    let mut b = use_signal::<f64>(|| 164. / 255. * 100.);

    use_effect(move || {
        let r = (255. / 100. * *r.read()).round() as u8;
        let g = (255. / 100. * *g.read()).round() as u8;
        let b = (255. / 100. * *b.read()).round() as u8;
        theme.write().colors.primary = format!("rgb({r}, {g}, {b})").into();
    });

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            main_align: "center",
            cross_align: "center",
            padding: "10",
            Switch {
                enabled: true,
                ontoggled: |_| {}
            }
            Slider {
                width: "fill",
                value: r(),
                onmoved: move |e| r.set(e),
            }
            Slider {
                width: "fill",
                value: g(),
                onmoved: move |e| g.set(e),
            }
            Slider {
                width: "fill",
                value: b(),
                onmoved: move |e| b.set(e),
            }
        }
    )
}
