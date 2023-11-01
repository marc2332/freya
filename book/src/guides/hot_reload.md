# Hot reload

Freya supports Dioxus hot reload, this means you can update the `layout` and `styling` of your app on the fly, without having to compile any rust code.

## Setup

Just before launching your app, you need to initialize the hot-reload context:

```rust, no_run
fn main() {

    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    launch(app);
}
```

That's it!