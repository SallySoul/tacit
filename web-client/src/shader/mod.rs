use std::cell::RefCell;
use web_sys::console::log_1;
use web_sys::*;

static SIMPLE_VS: &'static str = include_str!("./vertex_shader.vert");
static SIMPLE_FS: &'static str = include_str!("./fragment_shader.frag");

static FADE_BACKGROUND_VS: &'static str = include_str!("./fade_background.vert");
static FADE_BACKGROUND_FS: &'static str = include_str!("./fade_background.frag");

/// Identifiers for our different shaders
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderKind {
    Simple,
    FadeBackground,
}

pub struct FadeBackgroundShader {
    program: WebGlProgram,
    pub position_attribute: u32,
    pub color_attribute: u32,
}

pub struct SimpleShader {
    program: WebGlProgram,
    pub position_attribute: u32,
    pub object_transform_uniform: WebGlUniformLocation,
    pub color_uniform: WebGlUniformLocation,
}

pub struct ShaderSystem {
    pub fade_background_shader: FadeBackgroundShader,
    pub simple_shader: SimpleShader,
    active_program: RefCell<ShaderKind>,
}

impl ShaderSystem {
    pub fn new(gl_context: &WebGlRenderingContext) -> ShaderSystem {
        let fade_background_shader = {
            let program = create_program(&gl_context, FADE_BACKGROUND_VS, FADE_BACKGROUND_FS)
                .expect("Create Fade Background program");

            let position_attribute_signed = gl_context.get_attrib_location(&program, "position");

            if position_attribute_signed < 0 {
                log_1(&format!("Could not get FadeBackground position attribute").into());
                panic!("Could not get FadeBackground position attribute");
            }

            let color_attribute_signed = gl_context.get_attrib_location(&program, "a_color");

            if color_attribute_signed < 0 {
                log_1(&format!("Could not get FadeBackground color attribute").into());
                panic!("Could not get FadeBackground color attribute");
            }

            FadeBackgroundShader {
                program,
                position_attribute: position_attribute_signed as u32,
                color_attribute: color_attribute_signed as u32,
            }
        };

        let simple_shader = {
            let program = create_program(&gl_context, SIMPLE_VS, SIMPLE_FS)
                .expect("Create Fade Background program");

            let position_attribute_signed = gl_context.get_attrib_location(&program, "position");

            if position_attribute_signed < 0 {
                log_1(&format!("Could not get FadeBackground position attribute").into());
                panic!("Could not get FadeBackground position attribute");
            }

            let object_transform_uniform = gl_context.get_uniform_location(
                &program, 
                "object_transform"
            ).expect("Could not get uniform");

            let color_uniform = gl_context.get_uniform_location(
                &program, 
                "color"
            ).expect("Could not get uniform");

            SimpleShader {
                program,
                position_attribute: position_attribute_signed as u32,
                object_transform_uniform,
                color_uniform,
            }
        };

        ShaderSystem {
            fade_background_shader,
            simple_shader,
            active_program: RefCell::new(ShaderKind::Simple),
        }
    }

    pub fn use_program(&self, gl_context: &WebGlRenderingContext, shader_kind: ShaderKind) {
        if *self.active_program.borrow() != shader_kind {
            match shader_kind {
                ShaderKind::Simple =>  {
                    gl_context.use_program(Some(&self.simple_shader.program));
                }
                ShaderKind::FadeBackground => {
                    gl_context.use_program(Some(&self.fade_background_shader.program));
                }
            };
            *self.active_program.borrow_mut() = shader_kind;
        }
    }
}

fn create_program(
    gl_context: &WebGlRenderingContext,
    vert_shader_src: &str,
    frag_shader_src: &str,
) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(
        &gl_context,
        WebGlRenderingContext::VERTEX_SHADER,
        vert_shader_src,
    )?;
    let frag_shader = compile_shader(
        &gl_context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        frag_shader_src,
    )?;
    link_program(&gl_context, &vert_shader, &frag_shader)
}

/// Create a shader program using the WebGL APIs
fn compile_shader(
    gl_context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl_context
        .create_shader(shader_type)
        .ok_or_else(|| "Could not create shader".to_string())?;
    gl_context.shader_source(&shader, source);
    gl_context.compile_shader(&shader);

    if gl_context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl_context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
    }
}

/// Link a shader program using the WebGL APIs
fn link_program(
    gl_context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl_context
        .create_program()
        .ok_or_else(|| "Unable to create shader program".to_string())?;

    gl_context.attach_shader(&program, &vert_shader);
    gl_context.attach_shader(&program, &frag_shader);

    gl_context.link_program(&program);

    if gl_context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl_context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
    }
}
