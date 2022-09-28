use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MouseEvent;

type EventListener<T> = Closure<dyn FnMut(T)>;
pub type MouseEventListener = EventListener<MouseEvent>;

pub fn attach_mouse_event_handler(event_type: &str, selector: &str, listener: &MouseEventListener) -> bool {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Ok(elements) = document.query_selector_all(selector) {
        (0..elements.length()).all(|i|{
            if let Some(node) = elements.item(i) {
                node.add_event_listener_with_callback(event_type, listener.as_ref().unchecked_ref()).is_ok()
            } else {
                false
            }
        })
    } else {
        false
    }
}
