use dioxus::prelude::*;
use dioxus_resize_observer::use_resize_signal;

fn app(cx: Scope) -> Element {
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
        div { onmounted: move |event| resize.observer.mount(event.data), "Size: {width} x {height}" }
    )
}

fn main() {
    dioxus_web::launch(app);
}
