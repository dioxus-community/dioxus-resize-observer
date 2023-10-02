use dioxus::prelude::*;
use js_sys::Array as JsArray;
use wasm_bindgen::closure::Closure as JsClosure;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;

/// Hook to get an element's size, updating on changes.
pub fn use_size(cx: Scope) -> (Option<ResizeObserverEntry>, &Observer) {
    let size = use_ref(cx, || None);
    let observer = {
        to_owned![size];
        use_on_resize(cx, move |new_size| size.set(Some(new_size)))
    };

    (size.read().clone(), observer)
}

/// Hook to observe changes to an element's size.
pub fn use_on_resize(
    cx: Scope,
    mut handler: impl FnMut(ResizeObserverEntry) + 'static,
) -> &Observer {
    use_state(cx, || {
        Observer::new(move |size: ResizeObserverEntry| {
            handler(size);
        })
    })
}

/// Resize observer
pub struct Observer {
    closure: JsClosure<dyn FnMut(JsArray)>,
}

impl Observer {
    fn new(mut callback: impl FnMut(ResizeObserverEntry) + 'static) -> Self {
        let closure = JsClosure::<dyn FnMut(JsArray)>::new(move |entries: JsArray| {
            let entry = entries.at(0);
            let entry: ResizeObserverEntry = entry.dyn_into().unwrap();

            callback(entry);
        });

        Self { closure }
    }

    /// Mount the observer to a mounted element.
    pub fn mount(&self, element: &MountedData) {
        let raw_elem = element
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        ResizeObserver::new(self.closure.as_ref().unchecked_ref())
            .unwrap()
            .observe(raw_elem);
    }
}
