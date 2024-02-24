# Font Style

Learn how the font style attributes work.

- [`color`](#color)
- [`font_family`](#font_family)
- [`font_size`](#font_size)
- [`align`](#align)
- [`font_style`](#font_style)
- [`font_weight`](#font_weight)
- [`font_width`](#font_width)
- [`line_height`](#line_height)
- [`max_lines`](#max_lines)
- [`letter_spacing`](#letter_spacing)
- [`word_spacing`](#word_spacing)
- [`decoration`](#decoration)
- [`decoration_style`](#decoration_style)
- [`decoration_color`](#decoration_color)
- [`text_shadow`](#text_shadow)
- [`text_overflow`](#text_overflow)


### color

The `color` attribute let's you specify the color of the text.

You can learn about the syntax of this attribute in [`Color Syntax`](/guides/style.html#color-syntax).

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            color: "green",
            "Hello, World!"
        }
    )
}
```

Another example showing [inheritance](#inheritance):

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            color: "blue",
            label {
                "Hello, World!"
            }
        }
    )
}

```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), [`text`](/guides/elements.html#paragraph-and-text)

### font_family

With the `font_family` you can specify what font do you want to use for the inner text.

Limitation: Only fonts installed in the system are supported for now.

Example: 

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_family: "Inter",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), 

### font_size

You can specify the size of the text using `font_size`.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), [`text`](/guides/elements.html#paragraph-and-text)

### align

You can change the alignment of the text using the `align` attribute.

Accepted values: `center`, `end`, `justify`, `left`, `right`, `start`

Example

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            align: "right",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text), 

### font_style

You can choose a style for a text using the `font_style` attribute.

Accepted values: `upright` (default), `italic` and `oblique`.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_style: "italic",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### font_weight

You can choose a weight for a text using the `font_weight` attribute.

Accepted values:
- `invisible`
- `thin`
- `extra-light`
- `light`
- `normal` (default)
- `medium`
- `semi-bold`
- `bold`
- `extra-bold`
- `black`
- `extra-black`
- `50`
- `100`
- `200`
- `300`
- `400`
- `500`
- `600`
- `700`
- `800`
- `900`
- `950`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### font_width

You can choose a width for a text using the `font_width` attribute.

Accepted values:
- `ultra-condensed`
- `extra-condensed`
- `condensed`
- `normal` (default)
- `semi-expanded`
- `expanded`
- `extra-expanded`
- `ultra-expanded`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```


Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).


### line_height

Specify the height of the lines of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            lines_height: "3",
            "Hello, World! \n Hello, again!"
        }
    )
}
```

### max_lines

Determines the amount of lines that the text can have. It has unlimited lines by default.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            "Hello, World! \n Hello, World! \n Hello, world!" // Will show all three lines
        }
        label {
            max_lines: "2",
            "Hello, World! \n Hello, World! \n Hello, world!" // Will only show two lines
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text).

### letter_spacing

Specify the spacing between characters of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            letter_spacing: "10",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### word_spacing

Specify the spacing between words of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            word_spacing: "10",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### decoration

Specify the decoration in a text.

Accpted values:
- `underline`
- `line-through`
- `overline`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### decoration_style

Specify the decoration's style in a text.

Accpted values:
- `solid` (default)
- `double`
- `dotted`
- `dashed`
- `wavy`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_style: "dotted",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### decoration_color

Specify the decoration's color in a text.

You can learn about the syntax of this attribute in [`Color Syntax`](/guides/style.html#color-syntax).

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_color: "orange",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`paragraph`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### text_shadow

Specify the shadow of a text.

Syntax: `<x> <y> <size> <color>`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            text_shadow: "0 18 12 rgb(0, 0, 0)",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`text`](/guides/elements.html#paragraph-and-text), [`label`](/guides/elements.html#label).

### text_overflow

Determines how text is treated when it exceeds its [`max_lines`](#max_lines) count. By default uses the `clip` mode, which will cut off any overflowing text, with `ellipsis` mode it will show `...` at the end.

Accepted values:
- `clip` (default)
- `ellipsis`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            max_lines: "3",
            text_overflow: "ellipsis",
            "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
        }
    )
}
```

Compatible elements: [`label`](/guides/elements.html#label), [`paragraph`](/guides/elements.html#paragraph-and-text).
