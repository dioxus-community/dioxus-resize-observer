use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_signals::use_signal;

fn app(cx: Scope) -> Element {
    let element_ref = use_signal(cx, || None);
    let (width, height) = use_size(cx, element_ref);

    render!(div { onmounted: move |event| element_ref.set(Some(event.data)), "Size: {width} x {height}" })
}

fn main() {
    dioxus_web::launch(app);
}
