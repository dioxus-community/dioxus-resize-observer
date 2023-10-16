[![Discord Server](https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square)](https://discord.gg/sKJSVNSCDJ)

# dioxus-resize-observer
Resize observer hooks for [Dioxus 🧬](https://dioxuslabs.com/).

## Support
- **0.1.0 - Dioxus v0.4** 🧬 **git - Dioxus v0.5**
- Web renderer (WASM)

## Examples

## Signals
```rust
let resize = use_resize_signal(cx);
let (width, height) = resize
    .read()
    .as_ref()
    .map(|entry| {
        let rect = entry.content_rect();
        (rect.width(), rect.height())
    })
    .unwrap_or_default();

render!(
  div { 
    onmounted: move |event| {
      resize.observer.mount(&event)
    },
    "Size: {width} x {height}"
  }
)
```

## Observer
```rust
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
```

## License
MIT
