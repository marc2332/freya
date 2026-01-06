## Commands
Most of the commands necessart to work in this repo are in the `./justfile` file, you can run each command like this `just <command-name>`.

Examples:
- Run tests: `just tc`
- Run doc tests: `just d`
- Run torin tests: `just t-layout`.

## Architecture
All the rust crates are located in the `./crates` folder, all those starting with freya- are related to Freya, the others are used by Freya as well but are generic, for example Torin.
Rust examples are located in the `./examples` folder.
The `./website` folder contains the Astro website.

## Rust
- Avoid uwnrap() in library / examples code unless it is completely necessary, prefer to handle errors explicitely. Its fine in tests though.
- Don't use `super::` for imports, prefer `crate::`
- Dont put unnecessary comments, if you are adding/refactoring a feature prefer to write doc comments `///` with a proper but not big explanation.
