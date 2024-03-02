
# `bracket_ratatui`

[![Crates.io](https://img.shields.io/crates/v/bracket_ratatui.svg)](https://crates.io/crates/bracket_ratatui)
[![Documentation](https://docs.rs/bracket_ratatui/badge.svg)](https://docs.rs/bracket_ratatui/0.1.2/bracket_ratatui/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bracketengine/bracket/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/bracket_ratatui.svg)](https://crates.io/crates/bracket_ratatui)
[![Rust](https://github.com/gold-silver-copper/bracket_ratatui/workflows/CI/badge.svg)](https://github.com/gold-silver-copper/bracket_ratatui/actions)

This crate provides an [Ratatui](https://github.com/ratatui-org/ratatui) integration for the [bracket](https://github.com/bracketengine/bracket) game engine.
This is my first rust contribution so please help with pull requests, and be aware that it's a bit buggy and unoptimized :D Breaking changes are quite likely in the near future.

**Trying out:**

An example WASM project is live at https://gold-silver-copper.github.io/ 

**Features:**
- Desktop and web platforms support

## Dependencies

Just bracket and ratatui

## Usage

See the examples hello_bracket and demo (demo currently uses some weird unsafe stuff so its not very good to learn from)

`cargo run --example hello_bracket --release`
`cargo run --example demo --release`

## See also

For extra widgets:

https://github.com/ratatui-org/awesome-ratatui

## bracket support table

**Note:** if you're looking for a `bracket_ratatui` version that supports `main` branch of bracket, check out [open PRs](https://github.com/gold-silver-copper/bracket_ratatui/pulls), there's a great chance we've already started working on the future bracket release support.

| bracket | bracket_ratatui |
|------|-----------|
| 0.13 | 0.1      |
