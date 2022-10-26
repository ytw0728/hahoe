use std::{rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};

pub fn get_program(context: &WebGl2RenderingContext) -> Rc<WebGlProgram> {
    let vert_shader = crate::webgl::shader::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
        in vec4 a_position;
        in vec4 a_color;
        in vec3 a_normal;

        uniform mat4 u_matrix;

        out vec4 v_color;
        out vec3 v_normal;

        void main() {
          gl_Position = u_matrix * a_position;
          v_color = a_color;
          v_normal = a_normal;
        }
        "##,
    ).unwrap();

    let frag_shader = crate::webgl::shader::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;

        in vec4 v_color;
        in vec3 v_normal;

        out vec4 outColor;
        void main() {
          //outColor = v_color;
          outColor = vec4(0.2, 1.0, 0.2, 1.0);  //terrain 색상 지정 전까진 초록색 하드코딩
          vec3 normal = normalize(v_normal);
  
          vec3 normalizeDirection = normalize(vec3(0.5, 0.7, 0.2));
  
        // normal과 뒤집어진 빛의 방향을 내적해서 light값을 계산합니다.
        float light = dot(normal, normalizeDirection);
        // 알파를 제외한 색상값 부분을 light값과 곱해줍니다.
        outColor.rgb *= light; 
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
