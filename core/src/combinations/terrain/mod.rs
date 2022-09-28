use std::sync::mpsc;

use gui::interaction::{attach_mouse_event_handler, MouseEventListener};
use specs::{WorldExt, Builder, DispatcherBuilder};
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{combinations::{ Combination }, components, systems, resources::{physics::time::Time, self}};

pub struct TerrainCombination;

impl Combination for TerrainCombination {
    fn init<'world, 'a, 'b>(world: &'world mut specs::World) -> specs::Dispatcher<'a, 'b> {
        let appData = JsValue::from(web_sys::window().unwrap().get("appData").expect("window.appData <object> is undefined"));
        let tool = js_sys::Reflect::get( &appData, &JsValue::from_str("tool"))
                            .expect("window.appData.tool <{value: string}> is undefined");


        // // command queue ['cutting (150, 150)', 'read front 0']
        // let (sender, receiver) = mpsc::channel::<Vec<String>>();
        // // receiver가 command 읽어서 얘한테 쌓음.
        // // let vector = vec![];
        // receiver.
        let edit_terrain_handler: MouseEventListener = Closure::wrap(Box::new(move |e|{
            let tool = js_sys::Reflect::get(&tool, &JsValue::from_str("value")).unwrap().as_string().unwrap();
            if !tool.is_empty() {
                // sender.send(vec![String::from("")]).unwrap();
                // 얘는 안됨 ㅠㅠ
                // let mut time = world.write_resource::<resources::physics::time::Time>();
                // *time = resources::physics::time::Time {
                //     dt: time.dt,
                //     time: time.time + std::time::Duration::new(0, time.dt),
                // };
            };
        }));
        attach_mouse_event_handler("click", "#canvas", &edit_terrain_handler);
        edit_terrain_handler.forget();
        
        // components (register)
        world.register::<components::terrain::Terrain>();

        // entity
        world.create_entity().with(components::terrain::Terrain { bitmap: terrain::test_runner1().unwrap() }).build();
        
        DispatcherBuilder::new()
            .with(systems::renders::terrain::RenderTerrainSystem {}, "render_terrain_system", &[])
            .build()
    }
}
