use nalgebra::Matrix4;

use web_sys::{WebGl2RenderingContext, WebGlProgram, HtmlInputElement};
use terrain::model::pixel::Pixel;
use terrain::Mesh;

use rust_3d::*;

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

pub fn set_rectangle(context: &WebGl2RenderingContext, mesh: &Mesh, program: &WebGlProgram, camera_rad_inputs: &[HtmlInputElement; 4]) {
    if mesh.num_faces() == 0 {
        return;
    }

    // reasonable defaults?
    let mut width_ratio = 1f32;
    let mut height_ratio = 1f32;

    let start_width_ratio = -1f32;
    let start_height_ratio = -1f32;    

    

    if let Ok(bounding_box) = mesh.bounding_box_maybe() {
        let max = bounding_box.max_p();
        let min = bounding_box.min_p();
        let width = max.x - min.x;
        let height = max.y - min.y;

        width_ratio = 2. / width as f32;
        height_ratio = 2. / height as f32;
    }

    let mut vertices: Vec<f32> = Vec::with_capacity(mesh.num_faces() * 9);

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

    for face_id in 0..mesh.num_faces() {
        if let Ok(points) = mesh.face_vertices(FId{ val: face_id}) {
            for point in points {
                let x = start_width_ratio + point.x as f32 * width_ratio;
                let y = start_height_ratio + point.y as f32 * height_ratio;
                vertices.push(x);
                vertices.push(y);
                vertices.push(point.height as f32);
            }
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

