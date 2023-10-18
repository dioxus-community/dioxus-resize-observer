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
/// 
/// ## Examples
/// ```
/// use dioxus::prelude::*;
/// use dioxus_resize_observer::use_size;
/// use dioxus_use_mounted::use_mounted;
///
/// fn app(cx: Scope) -> Element {
///     let mounted = use_mounted(cx);
///     let (width, height) = use_size(cx, mounted);
///
///     render!(div {
///         onmounted: move |event| mounted.onmounted(event),
///         "Size: {width} x {height}"
///     })
/// }
/// ```
pub fn use_size<T>(cx: Scope<T>, mounted: UseMounted) -> (f64, f64) {
    let resize = use_resize(cx, mounted);
    let resize_ref = resize.read();
    resize_ref.unwrap_or_default()
}

/// Hook to get an element's resize events as a signal.
pub fn use_resize<T>(cx: Scope<T>, mounted: UseMounted) -> Signal<Option<(f64, f64)>> {
    let state_ref: Signal<Option<State>> = use_signal(cx, || None);
    let size_ref = use_signal(cx, || None);

    dioxus_signals::use_effect(cx, move || {
        if let Some(mounted) = mounted.signal.read().clone() {
            // Unmount the previous element, if it exists
            maybe_unobserve(state_ref);

            // Create a new resize observer with an entry event handler.
            let on_resize = Closure::<dyn FnMut(Array)>::new(move |entries: Array| {
                let entry = entries.at(0);
                let entry: ResizeObserverEntry = entry.dyn_into().unwrap();
                let rect = entry.content_rect();
                size_ref.set(Some((rect.width(), rect.height())));
            });
            let resize_observer = ResizeObserver::new(on_resize.as_ref().unchecked_ref()).unwrap();

            // Observe the raw element with the resize observer.
            let raw_elem = get_raw_element(&mounted);
            resize_observer.observe(raw_elem);

            // Update the current state.
            state_ref.set(Some(State {
                resize_observer,
                mounted,
                _on_resize: on_resize,
            }));
        } else {
            // Unmount the current element, if it exists
            maybe_unobserve(state_ref);
        }
    });

    size_ref
}

/// State of the hook.
struct State {
    /// JS resize observer.
    resize_observer: ResizeObserver,

    /// Currently mounted element data.
    mounted: Rc<MountedData>,

    /// Current closure handling resize observer events.
    _on_resize: Closure<dyn FnMut(Array)>,
}

/// Utility to get the raw element from its mounted data.
fn get_raw_element(mounted: &MountedData) -> &web_sys::Element {
    mounted
        .get_raw_element()
        .unwrap()
        .downcast_ref::<web_sys::Element>()
        .unwrap()
}

/// Attempt to unobserve an element, if it exists.
fn maybe_unobserve(state_ref: Signal<Option<State>>) {
    if let Some(state) = state_ref.write().take() {
        let raw_elem = get_raw_element(&state.mounted);
        state.resize_observer.unobserve(raw_elem);
    }
}
