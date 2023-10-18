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
use dioxus_signals::use_signal;

fn app(cx: Scope) -> Element {
    let element_ref = use_signal(cx, || None);
    let (width, height) = use_size(cx, element_ref);

    render!(div {
      onmounted: move |event| element_ref.set(Some(event.data)),
      "Size: {width} x {height}"
    })
}
```

## License
MIT
