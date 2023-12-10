# Theming

Freya has built-in support for Theming. 

> ⚠️ Currently, extending the base theme is not supported.

## Accessing the current theme

You can access the whole current theme via the `use_get_theme` hook.

```rust, no_run
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

## Custom default theme 
By default, the selected theme is `LIGHT_THEME`. You can use the alternative, `DARK_THEME`.

```rust, no_run
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

## Change theme globally

Changing the selected theme at runtime is possible by using the `use_theme` hook.

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        ThemeProvider {
            Component { }
        }
    )
}

#[allow(non_snake_case)]
fn Component(cx: Scope) -> Element {
    let theme = use_theme(cx);

    let onclick = |_| {
        *theme.write() = LIGHT_THEME;
    };

    render!(
        Button {
            onclick: onclick,
            label {
                "Use Light theme"
            }
        }
    )
}
```

## Change theme for an individual component

Most built-in components have their own theme "override."
You can specify which values to override like this:

```rust,no_run
fn app(cx: Scope) -> Element {
    render! {
        Button {
            theme: ButtonThemeWith {
                background: Some("blue").into(),
                font_theme: FontThemeWith {
                    Some("white").into(),
                    ..Default::default()
                },
                ..Default::default()
            },
            label { "I'm blue now" }
        }
    }
}
```

We need to use a different "type" of theme.
In the "ThemeWith" structs, each field is optional, so that the component knows what to override and
what to keep.
Additionally, we need to also spread `..Default::default`, to make all the other fields `None`.

To make this less verbose, you can use the `theme_with!` macro:

```rust,no_run
fn app(cx: Scope) -> Element {
    render! {
        Button {
            theme: theme_with!(ButtonTheme {
                background: "blue".into(),
                font_theme: theme_with!(FontTheme {
                    "white".into(),
                }),
            }),
            label { "I'm blue now" }
        }
    }
}
```

>️ ⚠️ The comma after the last field in the `theme_with!` macro is required.

As you can see, it removes the need for the "With" suffix, because that's already in the macro name.
More importantly, though, it wraps each file in a `Some`, and adds the spread.

## Custom theme

Themes can be built from scratch or extended from others, like here with `LIGHT_THEME`:

```rust, no_run
const CUSTOM_THEME: Theme = Theme {
    button: ButtonTheme {
        background: Cow::Borrowed("rgb(230, 0, 0)"),
        hover_background: Cow::Borrowed("rgb(150, 0, 0)"),
        font_theme: FontTheme {
            color: Cow::Borrowed("white"),
        },
        ..LIGHT_THEME.button
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
