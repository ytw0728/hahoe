use std::{sync::{Mutex, Arc}};

use gui::interaction::{attach_mouse_event_handler, MouseEventListener};
use specs::{WorldExt, Builder, DispatcherBuilder};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::console;

use crate::{combinations::{ Combination }, components, systems, resources::{physics::time::Time, self}, utils::store::Store};

pub struct TerrainCombination;

#[derive(Copy, Clone, Default, PartialEq)]
pub struct MyState {
    pub count: i32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Msg {
    TryAdd,
    AddSuccess,
    AddFailed,
    Noop
}

fn is_terminal(msg: Msg) -> bool {
    std::mem::discriminant(&Msg::Noop) == std::mem::discriminant(&msg)
}

fn state_reducer(state: MyState, msg: Msg) -> MyState {
    match msg {
        Msg::AddSuccess => MyState { count: state.count + 1 },
        _ => state
    }
}

fn try_inc_splitter(state: MyState, msg: Msg) -> Msg {
    match msg {
        Msg::TryAdd => {
            if state.count < 10 {
                Msg::AddSuccess
            } else {
                Msg::AddFailed
            }
        },
        _ => Msg::Noop,
    }
}
fn logger_mw(_state: MyState, msg: Msg) -> Msg {
    match msg {
        Msg::AddSuccess => {
            console::log_1(&JsValue::from_str("[LOG] successfully added."));
        },
        Msg::AddFailed => {
            console::log_1(&JsValue::from_str("[LOG] failed to add."));
        },
        Msg::TryAdd => {
            console::log_1(&JsValue::from_str("[LOG] try add."));
        },
        v => {
            console::log_1(&JsValue::from_str("[LOG] Terminal"));
        },
    };
    Msg::Noop
}
fn state_change_mw(state: MyState, msg: Msg) -> Msg {
    match msg {
        Msg::AddSuccess => {
            console::log_1(&JsValue::from_str(format!("MyState {{ count: {} }}", state.count).as_str()));
            Msg::Noop
        },
        _ => Msg::Noop
    }
}



impl Combination for TerrainCombination {
    fn init<'world, 'a, 'b>(world: &'world mut specs::World) -> specs::Dispatcher<'a, 'b> {
        let appData = JsValue::from(web_sys::window().unwrap().get("appData").expect("window.appData <object> is undefined"));
        let tool = js_sys::Reflect::get( &appData, &JsValue::from_str("tool"))
                            .expect("window.appData.tool <{value: string}> is undefined");

        let store = Arc::new(Mutex::new(Store::new(
            Box::new(state_reducer),
            Box::new(is_terminal),
        )));
        let mut cloned_store = Arc::clone(&store);
        let mut locked_store= cloned_store.lock().unwrap();
        locked_store.use_middleware(Box::new(logger_mw));
        locked_store.use_middleware(Box::new(try_inc_splitter));
        locked_store.use_middleware(Box::new(state_change_mw));

        locked_store.dispatch(Msg::TryAdd);

        let cur_count = locked_store.view().count;
        console::log_1(&JsValue::from_str(format!("current state count = {}", cur_count).as_str()));

        console::log_1(&JsValue::from_str("Backtrace state-change history 5 steps ->"));

        for state in locked_store.backtrace(5) {
            console::log_1(&JsValue::from_str(format!("MyState {{ count: {} }}", state.count).as_str()));
        }

        let t = store.clone();
        let edit_terrain_handler: MouseEventListener = Closure::wrap(Box::new(move |e|{
            let tool = js_sys::Reflect::get(&tool, &JsValue::from_str("value")).unwrap().as_string().unwrap();
            if !tool.is_empty() {
                let mut locked_t = t.lock().unwrap();
                locked_t.dispatch(Msg::TryAdd);
            };
        }));

        locked_store.dispatch(Msg::TryAdd);

        attach_mouse_event_handler("click", "#canvas", &edit_terrain_handler);
        edit_terrain_handler.forget();
        
        // components (register)
        world.register::<components::terrain::Terrain>();

        // entity
        world.create_entity().with(components::terrain::Terrain { bitmap: terrain::test_runner1().unwrap() }).build();
        
        DispatcherBuilder::new()
            .with(systems::renders::terrain::RenderTerrainSystem { store: store }, "render_terrain_system", &[])
            .build()
    }
}
