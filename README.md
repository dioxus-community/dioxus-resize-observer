[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/dioxus-community/dioxus-resize-observer#license)
[![Crates.io](https://img.shields.io/crates/v/dioxus-resize-observer.svg)](https://crates.io/crates/dioxus-resize-observer)
[![Docs](https://docs.rs/dioxus-resize-observer/badge.svg)](https://docs.rs/cdk-builder/latest/dioxus-resize-observer/)
[![CI](https://github.com/dioxus-community/dioxus-resize-observer/workflows/CI/badge.svg)](https://github.com/dioxus-community/dioxus-resize-observer/actions)
[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)

# dioxus-resize-observer
Resize observer hooks for [Dioxus 🧬](https://dioxuslabs.com/).

## Support
- Web renderer (WASM)
  - **0.1.0 - Dioxus v0.4**
  - **0.2.0 - Dioxus v0.5**

## Example
```rust
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

fn app() -> Element {
    let mounted = use_mounted();
    let (width, height) = use_size(mounted);

    render!(div {
      onmounted: move |event| mounted.onmounted(event),
      "Size: {width} x {height}"
    })
}
```

## License
MIT
