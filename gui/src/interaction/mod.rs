use web_sys::PointerEvent;

pub fn attach_event_handler(event_type: &str, element_id: &str, listener: &js_sys::Function) -> bool {
    let document = web_sys::window().unwrap().document().unwrap();
    match document.get_element_by_id(element_id) {
        Some(element) => {
            let event_wrapper: &dyn Fn(web_sys::PointerEvent) -> () = (&|e|{
                
            });
            element.add_event_listener_with_callback(event_type, ).is_ok()
        },
        None => false
    }
}


// fn event_wrapper(listener: &js_sys::Function) {
//     listener.(1,2,3);
// }