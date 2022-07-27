use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use nalgebra::{Matrix4};

pub struct Camera {

}

impl Camera {
    pub fn make_current_matrix() -> Matrix4<f32> {
        let document = web_sys::window().unwrap().document().unwrap();
        let ranges = [
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("x_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("y_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("z_range").unwrap())),
            HtmlInputElement::from(JsValue::from(document.get_element_by_id("d_range").unwrap())),
        ];

        let camera_rads = ranges.clone().map(|node| {
            let value = node.value();
            match value.parse::<f32>() {
                Ok(value) => value,
                Err(_) => 0.0_32
            }
        });
    
        let x = (camera_rads[0] - 50.0) / 50.0 * 2.0;
        let y = (camera_rads[1] - 50.0) / 50.0 * 2.0;
        let z = (camera_rads[2] - 50.0) / 50.0 * 2.0;
        let d = camera_rads[3];

        let default = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, d,
        );
    
        let rotate_x = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, x.cos(), -x.sin(), 0.0,
            0.0, x.sin(), x.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let rotate_y = Matrix4::new(
            y.cos(), 0.0, y.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -y.sin(), 0.0, y.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
    
        let rotate_z = Matrix4::new(
            z.cos(), -z.sin(), 0.0, 0.0,
            z.sin(), z.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        (&rotate_x * &rotate_y * &rotate_z * &default)
    }
}