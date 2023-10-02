use dioxus::prelude::*;
use js_sys::Array as JsArray;
use std::rc::Rc;
use wasm_bindgen::closure::Closure as JsClosure;
use wasm_bindgen::JsCast;
use web_sys::ResizeObserver;
use web_sys::ResizeObserverEntry;

/// Hook to get an element's size, updating on changes.
pub fn use_resize_observer(cx: Scope) -> (Option<ResizeObserverEntry>, &Observer) {
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
    let observer = use_state(cx, || {
        Observer::new(move |size: ResizeObserverEntry| {
            handler(size);
        })
    });

    let unmount_observer = observer.clone();
    use_on_unmount(cx, move || {
        if let Some((resize_observer, target)) =
            unmount_observer.resize_observer.borrow_mut().take()
        {
            let raw_elem = target
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();
            resize_observer.unobserve(raw_elem);
        }
    });

    observer
}

/// Resize observer
pub struct Observer {
    closure: JsClosure<dyn FnMut(JsArray)>,
    resize_observer: RefCell<Option<(ResizeObserver, Rc<MountedData>)>>,
}

impl Observer {
    fn new(mut callback: impl FnMut(ResizeObserverEntry) + 'static) -> Self {
        let closure = JsClosure::<dyn FnMut(JsArray)>::new(move |entries: JsArray| {
            let entry = entries.at(0);
            let entry: ResizeObserverEntry = entry.dyn_into().unwrap();

            callback(entry);
        });

        Self {
            closure,
            resize_observer: RefCell::default(),
        }
    }

    /// Mount the observer to a mounted element.
    pub fn mount(&self, element: Rc<MountedData>) {
        // Skip mounting if an observer is already stored.
        if self.resize_observer.borrow().is_some() {
            return;
        }

        // Get the current raw JS element.
        let raw_elem = element
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        // Create the ResizeObserver and observe the current element.
        let f = self.closure.as_ref().unchecked_ref();
        let resize_observer = ResizeObserver::new(f).unwrap();
        resize_observer.observe(raw_elem);

        // Set the current resize observer so it can be dropped later.
        *self.resize_observer.borrow_mut() = Some((resize_observer, element));
    }

    pub fn unmount(&self) {
        if let Some((resize_observer, target)) = &*self.resize_observer.borrow() {
            // Get the stored raw JS element.
            let raw_elem = target
                .get_raw_element()
                .unwrap()
                .downcast_ref::<web_sys::Element>()
                .unwrap();

            // Unobserve resizes.
            resize_observer.unobserve(raw_elem);
        }
    }
}
