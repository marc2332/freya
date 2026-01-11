# Agents

## Commands

Most of the commands necessary to work in this repo are in the `./justfile` file, you can run each command like this `just <command-name>`.

Examples:

- Format: `just f`
- Run tests: `just tc`
- Run doc tests: `just d`
- Run layout (torin) tests: `just t-layout`.

## Architecture

All the rust crates are located in the `./crates` folder, all those starting with freya- are related to Freya, the others are used by Freya as well but are generic, for example Torin.
Rust examples are located in the `./examples` folder.
The `./website` folder contains the Astro website.
Documentation is located in `./crates/freya/src/_docs`.

## Rust

- Avoid unwrap() in library / examples code unless it is completely necessary, prefer to handle errors explicitly. Its fine in tests though.
- Don't use `super::` for imports, prefer `crate::`
- Don't put unnecessary comments, if you are adding/refactoring a feature prefer to write doc comments `///` with a proper but not big explanation.
- Always implement the `KeyExt` trait for components to enable key-based reconciliation.
- Use `#[derive(PartialEq)]` for component structs to enable proper diffing and updates.
- Use `use freya_core::prelude::*;` to import common types and traits in component files.

## General instructions

- When working in one feature try to do as less commits as possible
  - If you commit something and then you change it again then redo the old commit
- When doing commits ask for confirmation
- Before committing and being finished make sure to run the formatter, linter and tests
- Never leave debug logs after finishin
- Never push to any branch, much less the `main` branch or using `--force`
- Never hardcode secrets or any other sensitive data
- Avoid creating temporary branches unless told
