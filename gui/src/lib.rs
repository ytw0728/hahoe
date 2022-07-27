pub mod camera;

use std::cell::RefCell;
use std::rc::Rc;
use camera::Camera;
use terrain::model::pixel::Pixel;
use terrain::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, console, HtmlInputElement, WebGlVertexArrayObject, WebGlBuffer};
use ndarray::{array, Array1};
use nalgebra::{Matrix4};
use std::f32;

const ZERO_HEIGHT: f32 = 0.0;
struct DrawingObject {
    pub program : WebGlProgram,
    pub vertex_array: WebGlVertexArrayObject,
    pub color_buffer_info: WebGlBuffer,
    pub vertex_buffer_info: WebGlBuffer,
    pub uniforms: Matrix4<f32>,
}

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
            color.push(bitmap[x+1][y+1].height as f32);
            color.push(0.2f32);
            color.push(1.0f32);

            color.push(0.2f32);
            color.push(bitmap[x+1][y].height as f32);
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

fn set_color2(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>) {
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
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

            color.push(0.2f32);
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

            color.push(0.2f32);
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

            color.push(0.2f32);
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

            color.push(0.2f32);
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

            color.push(0.2f32);
            color.push(0.2);
            color.push(1f32);
            color.push(0.5f32);

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

fn draw_terrain(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>, drawing_object: &DrawingObject) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

  //Set Color
  context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&drawing_object.color_buffer_info));
  
  let color_attribute_location = context.get_attrib_location(&drawing_object.program, "a_color");
  context.enable_vertex_attrib_array(color_attribute_location as u32);
  context.vertex_attrib_pointer_with_i32(
      color_attribute_location as u32,
      4,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );
    
    set_color(&context, &bitmap);

  context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&drawing_object.vertex_buffer_info));
  let position_attribute_location = context.get_attrib_location(&drawing_object.program, "a_position");
  context.enable_vertex_attrib_array(position_attribute_location as u32);
  context.vertex_attrib_pointer_with_i32(
      position_attribute_location as u32,
      3,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
  );

  set_rectangle(&context, &bitmap, &drawing_object);

}

fn draw_water(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>, drawing_object: &DrawingObject) {
    if bitmap.is_empty() {
        return;
    }
    if bitmap[0].is_empty() {
        return;
    }

  //Set Color
  context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&drawing_object.color_buffer_info));
  
  let color_attribute_location = context.get_attrib_location(&drawing_object.program, "a_color");
  context.enable_vertex_attrib_array(color_attribute_location as u32);
  context.vertex_attrib_pointer_with_i32(
      color_attribute_location as u32,
      4,
      WebGl2RenderingContext::FLOAT,
      false,
      0,
      0,
    );
    
    set_color2(&context, &bitmap);

    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&drawing_object.vertex_buffer_info));
    let position_attribute_location = context.get_attrib_location(&drawing_object.program, "a_position");
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
  
    set_rectangle(&context, &bitmap, &drawing_object);

}

fn set_rectangle(context: &WebGl2RenderingContext, bitmap: &Vec<Vec<Pixel>>, drawing_object: &DrawingObject) {
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

    let matrix_location = context.get_uniform_location(&drawing_object.program, "u_matrix");
    context.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, drawing_object.uniforms.as_slice());

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

    let terrain_vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&terrain_vao));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.enable(WebGl2RenderingContext::BLEND);
    context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

    let mut terrain_object = DrawingObject{
        program : program.clone(),
        vertex_array : terrain_vao,
        color_buffer_info : context.create_buffer().ok_or("Failed to create buffer")?,
        vertex_buffer_info : context.create_buffer().ok_or("Failed to create buffer")?,
        uniforms : Camera::make_current_matrix()
    };

    let water_vao = context
    .create_vertex_array()
    .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&water_vao));

    let mut water_object = DrawingObject{
        program : program.clone(),
        vertex_array : water_vao,
        color_buffer_info : context.create_buffer().ok_or("Failed to create buffer")?,
        vertex_buffer_info : context.create_buffer().ok_or("Failed to create buffer")?,
        uniforms : Camera::make_current_matrix()
    };

    let optbitmap = test_runner1();
    let bitmap = optbitmap.unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        terrain_object.uniforms = Camera::make_current_matrix();
        water_object.uniforms = Camera::make_current_matrix();
        draw_terrain(&context, &bitmap,  &terrain_object);
        
        let mut water_bitmap = (0..2)
                .map(|_| {
                    (0..2)
                        .map(|_| ->  Pixel { Pixel::make_dummy() })
                        .collect()
                })
                .collect();

        draw_water(&context, &water_bitmap, &water_object);
                
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
