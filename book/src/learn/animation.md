# Animation

Freya comes with a `use_animation` hook to create and manage animations in declarative way.

TODO

Example:
```rs
fn app() -> Element {
    let animation = use_animation(|ctx| {
        ctx.auto_start(true);
        ctx.with(AnimNum::new(0., 100.).time(50))
    });

    let width = animation.get().read().as_f32();

    rsx!(rect {
        width: "{width}",
        height: "100%",
        background: "blue"
    })
}