[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)

# dioxus-resize-observer
Resize observer hooks for [Dioxus 🧬](https://dioxuslabs.com/).

## Support
- **0.1.0 - Dioxus v0.4** 🧬 **git - Dioxus v0.5**
- Web renderer (WASM)

## Example
```rust
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let (width, height) = use_size(cx, mounted);

    render!(div {
      onmounted: move |event| mounted.onmounted(event),
      "Size: {width} x {height}"
    })
}
```

## License
MIT
