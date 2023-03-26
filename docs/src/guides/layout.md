# Layout

> Freya's layout is still somewhat limited.

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
- Pixels
- Percentage
- `calc()`

TODO

### direction

TODO

### display

TODO

### overflow

TODO