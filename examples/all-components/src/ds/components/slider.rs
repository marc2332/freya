use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsSlider() -> Element {
    let mut value = use_signal(|| 10.0);

    rsx!(Slider {
        value: *value.read(),
        onmoved: move |p| {
            value.set(p);
        }
    })
}
