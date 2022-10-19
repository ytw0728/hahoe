use gui::basics::GUI_BASICS;
use gui::dom::get_document;
use gui::webgl::buffer::init::{
    BufferDataFiller, BufferDataMaker, ColorBufferDataFiller, ColorBufferDataMaker,
    RectangleBufferDataMaker,
};
use specs::{Read, ReadStorage, System};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

pub struct RenderTerrainSystem;

impl<'a> System<'a> for RenderTerrainSystem {
    // TODO: resource (time) 사용법 전달드리고 나면, 제거하기 (여기선 필요없음.)
    type SystemData = (
        Read<'a, crate::resources::physics::time::Time>,
        ReadStorage<'a, crate::components::terrain::Terrain>,
    );
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        // MEMO: resource는 이렇게 쓰면 됩니다.
        let (time, terrain) = data;
        let document = get_document();

        for terrain in terrain.join() {
            let ranges = [
                HtmlInputElement::from(JsValue::from(
                    document.get_element_by_id("x_range").unwrap(),
                )),
                HtmlInputElement::from(JsValue::from(
                    document.get_element_by_id("y_range").unwrap(),
                )),
                HtmlInputElement::from(JsValue::from(
                    document.get_element_by_id("z_range").unwrap(),
                )),
                HtmlInputElement::from(JsValue::from(
                    document.get_element_by_id("d_range").unwrap(),
                )),
            ];

            GUI_BASICS
                .context
                .use_program(Some(&Rc::clone(&GUI_BASICS.program)));
            let colorBufferDataMaker = ColorBufferDataMaker {
                context: Rc::clone(&GUI_BASICS.context),
                program: Rc::clone(&GUI_BASICS.program),
            };
            let colorBufferData = colorBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let colorBufferDataFiller = ColorBufferDataFiller {
                context: Rc::clone(&GUI_BASICS.context),
                program: Rc::clone(&GUI_BASICS.program),
                buffer_data: Some(colorBufferData),
            };

            colorBufferDataFiller.bind_buffer();
            colorBufferDataFiller.fill_with_buffer_data();

            let rectangleBufferDataMaker = RectangleBufferDataMaker {
                context: Rc::clone(&GUI_BASICS.context),
                program: Rc::clone(&GUI_BASICS.program),
            };
            let rectangleBufferData = rectangleBufferDataMaker.make_buffer_data(&terrain.bitmap);
            let rectangleBufferDataLength = rectangleBufferData.len();
            let rectangleBufferDataFiller = gui::webgl::buffer::init::RectangleBufferDataFiller {
                context: Rc::clone(&GUI_BASICS.context),
                program: Rc::clone(&GUI_BASICS.program),
                buffer_data: Some(rectangleBufferData),
            };

            rectangleBufferDataFiller.bind_buffer();
            rectangleBufferDataFiller.fill_with_buffer_data();

            gui::webgl::buffer::update::set_uniform_matrix(
                &GUI_BASICS.context,
                &Rc::clone(&GUI_BASICS.program),
                &ranges,
            );
            gui::webgl::draw(&GUI_BASICS.context, (rectangleBufferDataLength / 3) as i32);
        }
    }
}
