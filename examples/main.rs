#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_resize_observer::use_size;

fn App(cx: Scope) -> Element {
    let (size, on_resize) = use_size(cx);

    render! {
      div {
        onmounted: move |event| {
          on_resize.mount(event);
        },
        "Size: {size.width} x {size.height}"
      }
    }
}

fn main() {
    dioxus_web::launch(App);
}