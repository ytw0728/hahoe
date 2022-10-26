use nalgebra::Matrix4;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

use crate::basics::GUI_BASICS;

pub struct Updater {}

impl Updater {
    pub fn new() -> Self {
        Updater {}
    }

    pub fn draw(&self, vertex_count: i32) {
        let context = &GUI_BASICS.context;
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertex_count);
    }

    pub fn clear(&self) {
        let context = &GUI_BASICS.context;
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn set_basic_uniform_matrix(&self) {
        let context = &GUI_BASICS.context;
        let program = &GUI_BASICS.program;
        let camera_rad_inputs = &GUI_BASICS.ranges;

        let camera_rads = camera_rad_inputs.as_ref().clone().map(|node| {
            let value = node.value();
            match value.parse::<f32>() {
                Ok(value) => value,
                Err(_) => 0.0_32,
            }
        });

        let x = (camera_rads[0] - 50.0) / 50.0 * 2.0;
        let y = (camera_rads[1] - 50.0) / 50.0 * 2.0;
        let z = (camera_rads[2] - 50.0) / 50.0 * 2.0;
        let d = camera_rads[3];

        let matrix_location = context.get_uniform_location(&Rc::clone(program), "u_matrix");
        let default = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, d,
        );

        let rotate_x = Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            x.cos(),
            -x.sin(),
            0.0,
            0.0,
            x.sin(),
            x.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );
        let rotate_y = Matrix4::new(
            y.cos(),
            0.0,
            y.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -y.sin(),
            0.0,
            y.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let rotate_z = Matrix4::new(
            z.cos(),
            -z.sin(),
            0.0,
            0.0,
            z.sin(),
            z.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        context.uniform_matrix4fv_with_f32_array(
            matrix_location.as_ref(),
            false,
            (&rotate_x * &rotate_y * &rotate_z * &default).as_slice(),
        );
    }
}
