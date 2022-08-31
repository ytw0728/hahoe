use std::rc::Rc;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, HtmlInputElement};
use wasm_bindgen::{JsValue, JsCast};

pub struct GuiBasics {
    pub canvas: Rc<HtmlCanvasElement>,
    pub context: Rc<WebGl2RenderingContext>,
    pub program: Rc<WebGlProgram>,
    pub ranges: Rc<[HtmlInputElement; 4]>,
}

impl GuiBasics {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let ranges = [
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("x_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("y_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("z_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("d_range").unwrap())),
        ];
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
    
        let program = crate::webgl::program::get_program(&context);

        GuiBasics {
            canvas: Rc::new(canvas),
            context: Rc::new(context),
            program: program,
            ranges: Rc::new(ranges),
        }
    }
}
