use std::cell::RefCell;
use std::rc::Rc;
use terrain::model::pixel::Pixel;
use terrain::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, console, HtmlInputElement};
use ndarray::{array, Array1};
use nalgebra::{Matrix4};
use std::f32;

const ZERO_HEIGHT: f32 = 0.0;

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> () {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn set_color(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
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
            color.push(0.2f32);
            color.push(bitmap[x][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x+1][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x][y+1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x][y+1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x+1][y].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x+1][y+1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

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

fn make_triangle_position(x1: f32, x2: f32, y1: f32, y2: f32, h1: f32, h2: f32, h3: f32, h4: f32) -> Vec<f32> {
    let mut vertices = Vec::<f32>::with_capacity(18);
    vertices.push(x1);
    vertices.push(y1);
    vertices.push(h1);

    vertices.push(x2);
    vertices.push(y1);
    vertices.push(h2);

    vertices.push(x1);
    vertices.push(y2);
    vertices.push(h3);

    vertices.push(x1);
    vertices.push(y2);
    vertices.push(h3);

    vertices.push(x2);
    vertices.push(y2);
    vertices.push(h4);

    vertices.push(x2);
    vertices.push(y1);
    vertices.push(h2);

    vertices
}

fn set_rectangle(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>, program: &WebGlProgram, camera_rad_inputs: &[HtmlInputElement; 4]) {
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
            vertices.append(&mut make_triangle_position(x1, x2, y1, y2, bitmap[x][y].height as f32, bitmap[x+1][y].height as f32, bitmap[x][y+1].height as f32, bitmap[x+1][y+1].height as f32));
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
        draw(&context, vert_count);
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let ranges = [
        HtmlInputElement::from(JsValue::from(document.get_element_by_id("x_range").unwrap())),
        HtmlInputElement::from(JsValue::from(document.get_element_by_id("y_range").unwrap())),
        HtmlInputElement::from(JsValue::from(document.get_element_by_id("z_range").unwrap())),
        HtmlInputElement::from(JsValue::from(document.get_element_by_id("d_range").unwrap())),
    ];
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.style().set_property("width", "100vw")?;
    canvas.style().set_property("height", "100vh")?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
        in vec4 a_position;
        in vec4 a_color;

        uniform mat4 u_matrix;

        out vec4 v_color;

        void main() {
          gl_Position = u_matrix * a_position;
          v_color = a_color;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;

        in vec4 v_color;

        out vec4 outColor;
        void main() {
          outColor = v_color;
        }
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let color_attribute_location = context.get_attrib_location(&program, "a_color");
    let position_attribute_location = context.get_attrib_location(&program, "a_position");

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.

    //Set Color
    let color_buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

    let optbitmap = test_runner1();
    let bitmap = optbitmap.unwrap();
    set_color(&context, &bitmap);

    context.enable_vertex_attrib_array(color_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        color_attribute_location as u32,
        4,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_rectangle(&context, &bitmap, &program, &ranges);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
