use dioxus::prelude::*;
use dioxus_signals::use_signal;
use dioxus_signals::Signal;
use dioxus_use_mounted::UseMounted;
use js_sys::Array;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;

/// Hook to get an element's size, updating on changes.
pub fn use_size<T>(cx: Scope<T>, mounted: UseMounted) -> (f64, f64) {
    let resize = use_resize(cx, mounted);
    let resize_ref = resize.read();
    resize_ref.unwrap_or_default()
}

/// Hook to get an element's resize events, updating on changes.
pub fn use_resize<T>(cx: Scope<T>, mounted: UseMounted) -> Signal<Option<(f64, f64)>> {
    let state_ref: Signal<Option<State>> = use_signal(cx, || None);
    let size_ref = use_signal(cx, || None);

    dioxus_signals::use_effect(cx, move || {
        if let Some(mounted) = mounted.signal.read().clone() {
            maybe_unobserve(state_ref);

            let on_resize = Closure::<dyn FnMut(Array)>::new(move |entries: Array| {
                let entry = entries.at(0);
                let entry: ResizeObserverEntry = entry.dyn_into().unwrap();
                let rect = entry.content_rect();
                size_ref.set(Some((rect.width(), rect.height())));
            });
            let resize_observer = ResizeObserver::new(on_resize.as_ref().unchecked_ref()).unwrap();

            let raw_elem = mounted
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();
            resize_observer.observe(raw_elem);

            state_ref.set(Some(State {
                on_resize,
                resize_observer,
                mounted,
            }));
        } else {
            maybe_unobserve(state_ref);
        }
    });

    size_ref
}

struct State {
    on_resize: Closure<dyn FnMut(Array)>,
    resize_observer: ResizeObserver,
    mounted: Rc<MountedData>,
}

fn maybe_unobserve(state_ref: Signal<Option<State>>) {
    if let Some(state) = state_ref.write().take() {
        let raw_elem = state
            .mounted
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();
        state.resize_observer.unobserve(raw_elem);
    }
}
