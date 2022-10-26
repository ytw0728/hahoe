mod combinations;
mod components;
mod resources;
mod systems;

use combinations::terrain::TerrainCombination;
use combinations::Combination;
use gui;
use gui::webgl::buffer::update::Updater;
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
    world.insert(resources::physics::time::Time {
        time: std::time::Duration::new(0, 0),
        dt: 16 * 1000 * 1000,
    });

    // dispatchers (init)
    // system들이 세팅된 dispatcher 들의 벡터
    let mut dispatchers = vec![TerrainCombination::init(&mut world)];

    play(world, dispatchers);
    Ok(())
}

fn play(mut world: World, mut dispatchers: Vec<Dispatcher<'static, 'static>>) {
    // request_animation_frame
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let updater = Updater {};

        updater.clear();
        updater.set_basic_uniform_matrix();

        // dispatcher 들을 순회하며 dispatch 메소드 실행. 현재 dispatchers 벡터에는 TerrainCombination 구조체만 들어가 있음.
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
