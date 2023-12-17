# Publishing

**Freya** produces a self-contained executable in `target/release`, so you can technically just distribute that.
However, you might want to create an installer instead. You can use executable packagers of your choice, but
for a more automated and "Rusty" version, you can use [**cargo-packager**](https://github.com/crabnebula-dev/cargo-packager), which is basically an abstraction
over executable packagers which you would have to setup yourself.

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

One very important configuration field is `formats`.
This is a list of installers that `cargo-packager` should generate, and by default, it's your current OS.
You can have a look at the list on [GitHub](https://github.com/crabnebula-dev/cargo-packager#supported-packages), or on the [API docs](https://docs.rs/cargo-packager/latest/cargo_packager/config/enum.PackageFormat.html).

# Optimizing

The ["Optimizing" chapter](https://dioxuslabs.com/learn/0.4/cookbook/optimizing) in the Dioxus docs applies in Freya too.
Note that WebAssembly-related tips are irrelevant.
