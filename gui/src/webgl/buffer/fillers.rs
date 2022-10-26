use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

use crate::basics::GUI_BASICS;

pub trait BufferDataFiller {
    fn bind_buffer(&self);

    fn fill_with_buffer_data(&self);
}
pub struct ColorBufferDataFiller {
    pub buffer_data: Option<Vec<f32>>,
}

impl ColorBufferDataFiller {
    pub fn new(buffer_data: Option<Vec<f32>>) -> Self {
        ColorBufferDataFiller { buffer_data }
    }
}

impl BufferDataFiller for ColorBufferDataFiller {
    fn bind_buffer(&self) {
        let context = Rc::clone(&GUI_BASICS.context);
        let program = Rc::clone(&GUI_BASICS.program);
        let buffer = context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let attribute_location = context.get_attrib_location(&program, "a_color");

        context.enable_vertex_attrib_array(attribute_location as u32);
        context.vertex_attrib_pointer_with_i32(
            attribute_location as u32,
            4,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }

    fn fill_with_buffer_data(&self) {
        unsafe {
            let context = Rc::clone(&GUI_BASICS.context);
            let array_buffer_view = js_sys::Float32Array::view(self.buffer_data.as_ref().unwrap());

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }
}

pub struct RectangleBufferDataFiller {
    pub buffer_data: Option<Vec<f32>>,
}

impl RectangleBufferDataFiller {
    pub fn new(buffer_data: Option<Vec<f32>>) -> Self {
        RectangleBufferDataFiller { buffer_data }
    }
}

impl BufferDataFiller for RectangleBufferDataFiller {
    fn bind_buffer(&self) {
        let context = Rc::clone(&GUI_BASICS.context);
        let program = Rc::clone(&GUI_BASICS.program);
        let buffer = context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        let position_attribute_location = context.get_attrib_location(&program, "a_position");
        context.enable_vertex_attrib_array(position_attribute_location as u32);
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }

    fn fill_with_buffer_data(&self) {
        unsafe {
            let context = Rc::clone(&GUI_BASICS.context);
            let array_buffer_view = js_sys::Float32Array::view(self.buffer_data.as_ref().unwrap());

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }
}
