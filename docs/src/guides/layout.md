# Layout

Learn how the layout attributes work.

> ⚠️ Freya's layout is still somewhat limited.

### width & height
All elements support both `width` and `height` attributes.

##### Usage

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            width: "15",
            height: "50",
        }
    )
}
```

### min_width & min_height

`rect` and `container` support specifying a minimum width and height, this can be useful if you use it alongside a percentage for the target size.

##### Usage

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            min_width: "100",
            min_height: "100",
            width: "50%",
            height: "50%",
        }
    )
}
```

### max_width & max_height

`rect` and `container` support specifying a maximum width and height.

##### Usage

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            max_width: "50%",
            max_height: "50%",
            width: "500",
            height: "500",
        }
    )
}
```

### Units

#### Static Values

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "50",
            height: "33"
        }
    )
}
```

#### Percentages
Relative percentage to the parent equivalent value.
```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "50%", // Half the window
            height: "75%" // 3/4 the window
        }
    )
}
```

#### `calc()`

For more complex logic you can use the `calc()` function.

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "calc(33% - 60 + 15%)",
            height: "calc(100% - 10)"
        }
    )
}
```

### direction

Control how the inner elements will be stacked, possible values are `horizontal`, `vertical` (default) or `both` (default for text elements, e.g label, paragraph, text, etc).

##### Usage

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            rect {
                width: "100%",
                height: "50%",
                background: "red"
            },
            rect {
                width: "100%",
                height: "50%",
                background: "green"
            }
        }
    )
}
```

### display

Control how the inner elements are displayed, possible values are `normal` (default) or `center`.

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "both",
            display: "center",
            rect {
                width: "50%",
                height: "50%",
                background: "red"
            },
        }
    )
}
```

### overflow

This is the key difference between `rect` and `container` elements. `rect` will still render any overflow it has, in the other hand, `container` will clip any content overflowing its bounds.