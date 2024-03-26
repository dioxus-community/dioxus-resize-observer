use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

fn app() -> Element {
    let mounted = use_mounted();
    let size = use_size(mounted);

    rsx!(
        div { onmounted: move |event| mounted.onmounted(event), "Size: {size.width()} x {size.height()}" }
    )
}

fn main() {
    dioxus_web::launch::launch_cfg(app, Default::default());
}
