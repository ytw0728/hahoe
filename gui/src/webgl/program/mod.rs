use std::{rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};

pub fn get_program(context: &WebGl2RenderingContext) -> Rc<WebGlProgram> {
    let vert_shader = crate::webgl::shader::compile_shader(
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
    ).unwrap();

    let frag_shader = crate::webgl::shader::compile_shader(
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
    ).unwrap();

    let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object").unwrap();
    context.bind_vertex_array(Some(&vao));
    
    Rc::new(program)
}


fn link_program(
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
