use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    let size = use_size(cx, mounted);

    render!(
        div { onmounted: move |event| mounted.onmounted(event), "Size: {size.width()} x {size.height()}" }
    )
}

fn main() {
    dioxus_web::launch(app);
}
