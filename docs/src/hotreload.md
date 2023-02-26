# Hot reload

Just before launching your app, you need to initialize the hot-reload context:

```rust
fn main() {

    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());

    launch(app);
}
```