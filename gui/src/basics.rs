use std::rc::Rc;

use lazy_static::lazy_static;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlInputElement, WebGl2RenderingContext, WebGlProgram};

use crate::{
    dom::{get_canvas, get_document},
    webgl::program::get_vertex_array_object,
};

const CANVAS_ID: &str = "canvas";

// TODO: GuiBasics::new를 thread safe하게 변경해 전역 변수 제거해보기.
lazy_static! {
    pub static ref GUI_BASICS: GuiBasics = GuiBasics::new();
}

unsafe impl Send for GuiBasics {}
unsafe impl Sync for GuiBasics {}
/** 전역의 GUI_BASICS를 대신 사용해주세요. */
pub struct GuiBasics {
    pub canvas: Rc<HtmlCanvasElement>,
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
    pub ranges: Rc<[HtmlInputElement; 4]>,
}

impl GuiBasics {
    pub fn new() -> Self {
        let document = get_document();
        let canvas = get_canvas(CANVAS_ID);

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
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();

        let program = crate::webgl::program::get_program(&context);
        context.use_program(Some(&program));

        let vertex_array_object = get_vertex_array_object(&context);

        context.bind_vertex_array(Some(&vertex_array_object));

        GuiBasics {
            canvas: Rc::new(canvas),
            context: Rc::new(context),
            program: Rc::new(program),
            ranges: Rc::new(ranges),
        }
    }
}
