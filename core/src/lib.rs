pub mod combinations;
pub mod components;
pub mod resources;
pub mod systems;

use gui;
use terrain;

use std::cell::RefCell;
use std::rc::Rc;

use specs::{Builder, RunNow, World, WorldExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use web_sys::console;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let mut world = World::new();
    let basics = Rc::new(gui::GuiBasics::new());

    // resources (bind)
    world.insert(resources::physics::time::Time {
        time: std::time::Duration::new(0, 0),
        dt: 16 * 1000 * 1000,
    });

    // components (register)
    world.register::<components::terrain::Terrain>();

    // entities (build)
    let terrain = world
        .create_entity()
        .with(components::terrain::Terrain {
            bitmap: terrain::test_runner1().unwrap(),
        })
        .build();

    // systems (init)
    let mut render_terrain_system = systems::renders::terrain::RenderTerrainSystem {};

    // request_animation_frame
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        gui::webgl::clear(&basics.context);
        render_terrain_system.run_now(&world);
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
    Ok(())
}
