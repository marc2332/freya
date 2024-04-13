# Publishing

> ⚠️ Before publishing, you should consider removing insecure metadata.
> For example, images might have EXIF location data in them.

**Freya** produces a self-contained executable in `target/release`, so you can technically distribute that.
However, you might want to create an installer instead. You can use executable packagers of your choice, but
for a more automated and "Rusty" version, you can use [**cargo-packager**](https://github.com/crabnebula-dev/cargo-packager), which is basically an abstraction
over executable packagers, which you would have to set up yourself.

There is an [example](https://github.com/marc2332/freya/tree/main/examples/installer) you can check out.

## `cargo-packager` installation

Run:

```
cargo install cargo-packager --locked
```

## Usage

Add this to your `Cargo.toml`:

```
[package.metadata.packager]
before-packaging-command = "cargo build --release" # Before packaging, packager will run this command.
product-name = "My App" # By default, the crate name will be shown, but you probably prefer "My App" over "my-app".
```

And run:

```
cargo packager --release
```

And there you go! You should now have an installer in `target/release` for your current OS.
To publish your app on a different OS, see the next section, [Configuration](#configuration).

## Configuration

We used a very bare-bones example, so make sure to check out all configuration options in the [Config struct](https://docs.rs/cargo-packager/latest/cargo_packager/config/struct.Config.html)
in the `cargo-packager` API docs. Note that underscores should be hyphens when you use TOML.

One crucial configuration field is `formats`.
This is a list of installers that `cargo-packager` should generate, and by default, it's your current OS.
You can have a look at the list on [GitHub](https://github.com/crabnebula-dev/cargo-packager#supported-packages), or on the [API docs](https://docs.rs/cargo-packager/latest/cargo_packager/config/enum.PackageFormat.html).

### Changing the executable icon on Windows

`cargo-packager` will change the icon for platforms other than Windows using the [`icons`](https://docs.rs/cargo-packager/latest/cargo_packager/config/struct.Config.html#structfield.icons)
field, but it does not do it on Windows (yet?).

Anyway, the `cargo-packager` team recommends using [`winresource`](https://crates.io/crates/winresource)
(as opposed to [`winres`](https://crates.io/crates/winres) which is not maintained).
Before using it, make sure that you have the requirements that are listed on its page.

Add it to your build dependencies in `Cargo.toml`:

```toml
[build-dependencies]
winresource = "0.1.7"
```

And add this to your `build.rs` file (make sure you link it in your `Cargo.toml`):

```rs
// Change this to your icon's location
const ICON: &str = "assets/icons/icon.ico";

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon(ICON);
        res.compile().unwrap();
    }
}
```

To convert more common formats like `.png` or `.jpg` to an `.ico`, you can use [imagemagick](https://imagemagick.org).
Once installed, run `magick convert your_icon.png icon.ico`.

# Optimizing

The ["Optimizing" chapter](https://dioxuslabs.com/learn/0.5/cookbook/optimizing) in the Dioxus docs applies in Freya too.
Note that WebAssembly-related tips are irrelevant.
