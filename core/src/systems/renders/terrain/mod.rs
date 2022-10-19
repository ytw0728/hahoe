use std::{sync::{Arc, Mutex}, borrow::Borrow};

use gui::basics::GUI_BASICS;
use specs::{Read, ReadStorage, System};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::{utils::store::Store, combinations::terrain::{MyState, Msg}};

pub struct RenderTerrainSystem {
    pub store: Arc<Mutex<Store<MyState, Msg>>>
}

impl<'a> System<'a> for RenderTerrainSystem {
    // TODO: resource (time) 사용법 전달드리고 나면, 제거하기 (여기선 필요없음.)
    type SystemData = (Read<'a, crate::resources::physics::time::Time>, ReadStorage<'a, crate::components::terrain::Terrain>);
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        // MEMO: resource는 이렇게 쓰면 됩니다.
        let (time,  terrain) = data;
        let mut locked_store = self.store.lock().unwrap();
        console::log_1(&JsValue::from_f64(f64::from(locked_store.view().count)));
        // locked_store.dispatch(Msg::TryAdd);
        

        for terrain in terrain.join() {
            gui::webgl::buffer::init::bind_color_buffer(&GUI_BASICS.context, &GUI_BASICS.program);
            gui::webgl::buffer::update::set_color(&GUI_BASICS.context, &terrain.bitmap);
            gui::webgl::buffer::init::bind_vertex_buffer(&GUI_BASICS.context, &GUI_BASICS.program);
            gui::webgl::buffer::update::set_rectangle(&GUI_BASICS.context, &terrain.bitmap, &GUI_BASICS.program, &GUI_BASICS.ranges);
        }
    }
}
