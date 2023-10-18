use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let (width, height) = use_size(cx, mounted);

    render!(div { onmounted: move |event| mounted.onmounted(event), "Size: {width} x {height}" })
}

fn main() {
    dioxus_web::launch(app);
}
