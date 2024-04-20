
 `bevy_ratatui`

[![Crates.io](https://img.shields.io/crates/v/bevy_ratatui.svg)](https://crates.io/crates/bevy_ratatui)
[![Documentation](https://docs.rs/bevy_ratatui/badge.svg)](https://docs.rs/bevy_ratatui/0.2.0/bevy_ratatui/index.html)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/bevy_ratatui.svg)](https://crates.io/crates/bevy_ratatui)
[![Rust](https://github.com/gold-silver-copper/bevy_ratatui/workflows/CI/badge.svg)](https://github.com/gold-silver-copper/bevy_ratatui/actions)

# THIS IS DEPRECATED USE MY OTHER CRATE ratframe or on github ratatui_egui_wasm
# it can be combined with bevy_egui to create a much better experience than with this old crate
https://github.com/gold-silver-copper/ratatui_egui_wasm
https://github.com/gold-silver-copper/ratatui_egui_wasm
https://github.com/gold-silver-copper/ratatui_egui_wasm

This crate provides an [Ratatui](https://github.com/ratatui-org/ratatui) integration for the [Bevy](https://github.com/bevyengine/bevy) game engine.
Please help with pull requests, and be aware that it's a bit buggy and unoptimized :D Breaking changes are quite likely in the near future. PLEASE LOOK AT THE EXAMPLES FOLDER 

**Trying out:**

An example WASM project is live at https://gold-silver-copper.github.io/ 

**Features:**
- Desktop and web platforms support

## Dependencies

Just bevy and ratatui

## Usage

See the examples hello_bevy and demo (demo currently uses some weird unsafe stuff so its not very good to learn from)

`cargo run --example hello_bevy --release`
`cargo run --example demo --release`

## See also


For extra widgets:

https://github.com/ratatui-org/awesome-ratatui

## Bevy support table

**Note:** if you're looking for a `bevy_ratatui` version that supports `main` branch of Bevy, check out [open PRs](https://github.com/gold-silver-copper/bevy_ratatui/pulls), there's a great chance we've already started working on the future Bevy release support.

| bevy | bevy_ratatui |
|--------|---------|
| 0.13.0 | 0.1 0.2 |
