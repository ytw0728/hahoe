mod resources;
mod components;
mod systems;
mod combinations;

use combinations::Combination;
use combinations::terrain::TerrainCombination;
use gui;
use gui::basics::GUI_BASICS;
use gui::interaction::MouseEventListener;
use gui::interaction::attach_mouse_event_handler;

use specs::Dispatcher;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::window;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use specs::{World, WorldExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let mut world = World::new();

    let appData = JsValue::from(window().unwrap().get("appData").expect("window.appData <object> is undefined"));
    let tool = js_sys::Reflect::get( &appData, &JsValue::from_str("tool"))
                            .expect("window.appData.tool <{value: string}> is undefined");

    let widget_click_handler: MouseEventListener = Closure::wrap(Box::new(move |e|{
        if let Some(target) = e.target().unwrap().dyn_ref::<Element>() {
            js_sys::Reflect::set(&tool, &JsValue::from_str("value"), &JsValue::from_str(target.id().as_str())).unwrap();
            let canvas =  window().unwrap().document().unwrap().query_selector("#canvas").unwrap().unwrap();
            if !target.id().is_empty() {
                canvas.set_class_name("modifying");
            } else {
                canvas.set_class_name("");
            }
        }
    }));
    attach_mouse_event_handler("click", ".widgets", &widget_click_handler);
    widget_click_handler.forget();


    // resources (bind)
    world.insert(resources::physics::time::Time { time: std::time::Duration::new(0, 0), dt: 16 * 1000 * 1000});

    // dispatchers (init)
    let dispatchers = vec![
        TerrainCombination::init(&mut world),
    ];

    play(world, dispatchers);
    Ok(())
}

fn play(mut world: World, mut dispatchers: Vec<Dispatcher<'static, 'static>>) {
    // request_animation_frame
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        gui::webgl::clear(&GUI_BASICS.context);
        for dispatcher in dispatchers.iter_mut() {
            dispatcher.dispatch(&world);    
        }
        world.maintain();

        // MEMO: update resource, time
        let mut time = world.write_resource::<resources::physics::time::Time>();
        *time = resources::physics::time::Time {
            dt: time.dt,
            time: time.time + std::time::Duration::new(0, time.dt),
        };
        
        gui::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    gui::request_animation_frame(g.borrow().as_ref().unwrap());
}