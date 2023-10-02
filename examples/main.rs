#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_resize_observer::use_size;

fn App(cx: Scope) -> Element {
    let (event, observer) = use_size(cx);
    let (width, height) = event
        .map(|entry| {
            let rect = entry.content_rect();
            (rect.width(), rect.height())
        })
        .unwrap_or_default();

    render! {div { onmounted: move |event| { observer.mount(&event) }, "Size: {width} x {height}" }}
}

fn main() {
    dioxus_web::launch(App);
}
