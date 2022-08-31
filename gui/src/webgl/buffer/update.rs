use nalgebra::Matrix4;

use web_sys::{WebGl2RenderingContext, WebGlProgram, HtmlInputElement};
use terrain::model::pixel::Pixel;

pub fn set_color(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

    let width = bitmap.len() - 1;
    let height = bitmap[0].len() - 1;
    let mut color = Vec::<f32>::with_capacity((width * height * 24) as usize);

    for x in 0..width{
        for y in 0..height{
            color.push(0.2f32); color.push(bitmap[x][y].height as f32); color.push(0.2f32); color.push(1.0f32);
            color.push(0.2f32); color.push(bitmap[x+1][y].height as f32); color.push(0.2f32); color.push(1.0f32);
            color.push(0.2f32); color.push(bitmap[x][y+1].height as f32); color.push(0.2f32); color.push(1.0f32);
            color.push(0.2f32); color.push(bitmap[x][y+1].height as f32); color.push(0.2f32); color.push(1.0f32);
            color.push(0.2f32); color.push(bitmap[x+1][y].height as f32); color.push(0.2f32); color.push(1.0f32);
            color.push(0.2f32); color.push(bitmap[x+1][y+1].height as f32); color.push(0.2f32); color.push(1.0f32);
        }
    }

    unsafe {
        let color_array_buf_view = js_sys::Float32Array::view(&color);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn set_rectangle(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>, program: &WebGlProgram, camera_rad_inputs: &[HtmlInputElement; 4]) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

    let width = bitmap.len() - 1;
    let height = bitmap[0].len() - 1;

    let width_ratio = 2. / width as f32;
    let height_ratio = 2. / height as f32;

    let start_width_ratio = -1.;
    let start_height_ratio = -1.;

    let mut vertices = Vec::<f32>::with_capacity((width * height * 18) as usize);

    let camera_rads = camera_rad_inputs.clone().map(|node| {
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

    let matrix_location = context.get_uniform_location(program, "u_matrix");

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

    context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, (&rotate_x * &rotate_y * &rotate_z * &default).as_slice());

    for x in 0..width {
        for y in 0..height {
            let x1 = start_width_ratio + x as f32 * width_ratio;
            let x2 = start_width_ratio + (x + 1) as f32 * width_ratio;
            let y1 = start_height_ratio + y as f32 * height_ratio;
            let y2 = start_height_ratio + (y + 1) as f32 * height_ratio;
            vertices.append(
                &mut make_triangle_position(
                    x1, x2, y1, y2,
                    bitmap[x][y].height as f32,
                    bitmap[x+1][y].height as f32,
                    bitmap[x][y+1].height as f32,
                    bitmap[x+1][y+1].height as f32
                )
            );
        }
    }

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vert_count = (vertices.len() / 3) as i32;
        crate::webgl::draw(&context, vert_count);
    }
}

fn make_triangle_position(x1: f32, x2: f32, y1: f32, y2: f32, h1: f32, h2: f32, h3: f32, h4: f32) -> Vec<f32> {
    let mut vertices = Vec::<f32>::with_capacity(18);
    vertices.push(x1); vertices.push(y1); vertices.push(h1);
    vertices.push(x2); vertices.push(y1); vertices.push(h2);
    vertices.push(x1); vertices.push(y2); vertices.push(h3);
    vertices.push(x1); vertices.push(y2); vertices.push(h3);
    vertices.push(x2); vertices.push(y2); vertices.push(h4);
    vertices.push(x2); vertices.push(y1); vertices.push(h2);
    vertices
}
