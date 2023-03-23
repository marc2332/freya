# Theming

Freya has built-in support for Theming. 

> Currently it does not support custom keys, only the already defined.

### Accessing the current theme
You can access the whole current theme via the `use_get_theme` hook.

```rust
fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            Component { }
        }
    )
}

#[allow(non_snake_case)]
fn Component(cx: Scope) -> Element {
    let theme = use_get_theme(cx);

    let button_theme = &theme.button;

    render!(
        rect {
            background: "{button_theme.background}",
        }
    )
}
```

### Changing the current theme
By default, the selected theme is `DARK_THEME`. You use the alternative, `LIGHT_THEME` or any you want.

```rust
fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            theme: LIGHT_THEME,
            Component { }
        }
    )
}

#[allow(non_snake_case)]
fn Component(cx: Scope) -> Element {
    let theme = use_get_theme(cx);

    let button_theme = &theme.button;

    render!(
        rect {
            background: "{button_theme.background}",
        }
    )
}
```

### Custom theme

TODO