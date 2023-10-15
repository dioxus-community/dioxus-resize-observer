[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)

# dioxus-resize-observer
Resize observer hooks for [Dioxus 🧬](https://dioxuslabs.com/).

## Support
- **Dioxus v0.4** 🧬
- Web renderer (WASM)

## Example

```rust
fn App(cx: Scope) -> Element {
    let (event, observer) = use_size(cx);
    let (width, height) = event
        .map(|entry| {
            let rect = entry.content_rect();
            (rect.width(), rect.height())
        })
        .unwrap_or_default();

    render!(
      div { 
        onmounted: move |event| {
          observer.mount(&event)
        },
        "Size: {width} x {height}"
      }
    )
}
```

MIT License