use dioxus::prelude::*;
use js_sys::Array as JsArray;
use wasm_bindgen::closure::Closure as JsClosure;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;

pub fn use_size(cx: Scope) -> (Size, UseState<OnResize>) {
    let size = use_ref(cx, Size::default);

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: Size| {
            size.set(new_size);
        })
    });

    (size.read().clone(), on_resize.clone())
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub struct OnResize {
    closure: JsClosure<dyn FnMut(JsArray)>,
}

impl OnResize {
    fn new(mut callback: impl FnMut(Size) + 'static) -> Self {
        let closure = JsClosure::<dyn FnMut(JsArray)>::new(move |entries: JsArray| {
            let entry = entries.at(0);
            let entry: ResizeObserverEntry = entry.dyn_into().unwrap();
            let rect = entry.content_rect();

            let size = Size {
                width: rect.width(),
                height: rect.height(),
            };
            callback(size);
        });

        Self { closure }
    }

    pub fn mount(&self, event: Event<MountedData>) {
        let element = event
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        ResizeObserver::new(self.closure.as_ref().unchecked_ref())
            .unwrap()
            .observe(element);
    }
}
