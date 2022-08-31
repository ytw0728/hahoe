mod resources;
mod components;
mod systems;
mod combinations;

use combinations::Combination;
use combinations::terrain::TerrainCombination;
use gui;
use gui::basics::GUI_BASICS;
use specs::Dispatcher;

use std::cell::RefCell;
use std::rc::Rc;

use specs::{World, WorldExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let mut world = World::new();
    // resources (bind)
    world.insert(resources::physics::time::Time { time: std::time::Duration::new(0, 0), dt: 16 * 1000 * 1000});

    // dispatchers (init)
    let mut dispatchers = vec![
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