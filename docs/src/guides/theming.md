# Theming

Freya has built-in support for Theming. 

> Currently it does not support extending the base theme.

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

Themes can be built from scratch or extended from others, like here with `LIGHT_THEME`:

```rust

const CUSTOM_THEME: Theme = Theme {
    button: ButtonTheme {
        background: "rgb(230, 0, 0)",
        hover_background: "rgb(150, 0, 0)",
        font_theme: FontTheme { color:  "white" }
    },
    ..LIGHT_THEME
};

fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            theme: CUSTOM_THEME,
            rect {
                width: "100%",
                height: "100%",
                Button {
                    label {
                        "Report"
                    }
                }
            }
        }
    )
}
```