use nalgebra::Matrix4;
use web_sys::{WebGlProgram, WebGlVertexArrayObject, WebGlBuffer};

pub struct DrawingObject {
    pub program : WebGlProgram,
    pub vertex_array: WebGlVertexArrayObject,
    pub color_buffer_info: WebGlBuffer,
    pub vertex_buffer_info: WebGlBuffer,
    pub uniforms: Matrix4<f32>,
}