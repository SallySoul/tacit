use std::cell::RefCell;
use web_sys::console::log_1;
use web_sys::*;

static SIMPLE_VS: &'static str = include_str!("./simple.vert");
static SIMPLE_FS: &'static str = include_str!("./simple.frag");
static FADE_BACKGROUND_VS: &'static str = include_str!("./fade_background.vert");
static FADE_BACKGROUND_FS: &'static str = include_str!("./fade_background.frag");
static BILLBOARD_VS: &'static str = include_str!("./billboard.vert");

/// Identifiers for our different shaders
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderKind {
    Simple,
    FadeBackground,
    BillBoard,
}

/// This shader doesn't change the vertex locations, but does have color as an attribute
/// thus useful for background fades.
pub struct FadeBackgroundShader {
    program: WebGlProgram,
    pub position_attribute: u32,
    pub color_attribute: u32,
}

/// This shader applies a world coordinate to clipspace transform on the vertices
/// and applies a uniform color. Useful for lines and points and little else.
pub struct SimpleShader {
    program: WebGlProgram,
    pub position_attribute: u32,
    pub object_transform_uniform: WebGlUniformLocation,
    pub color_uniform: WebGlUniformLocation,
}

/// This shader is used for "billboard" geometry, that is 2D shapes that always need
/// to face the camera. Color is an attribute
pub struct BillBoardShader {
    program: WebGlProgram,
    pub board_position_attribute: u32,
    pub board_center_attribute: u32,
    pub color_uniform: WebGlUniformLocation,
    pub camera_up_uniform: WebGlUniformLocation,
    pub camera_right_uniform: WebGlUniformLocation,
    pub worldspace_transform_uniform: WebGlUniformLocation,
}

pub struct ShaderSystem {
    pub fade_background_shader: FadeBackgroundShader,
    pub simple_shader: SimpleShader,
    pub billboard_shader: BillBoardShader,
    active_program: RefCell<ShaderKind>,
}

impl ShaderSystem {
    pub fn new(gl_context: &WebGlRenderingContext) -> ShaderSystem {
        let fade_background_shader = {
            let program = create_program(&gl_context, FADE_BACKGROUND_VS, FADE_BACKGROUND_FS)
                .expect("Create Fade Background program");

            let position_attribute = get_attribute_pointer(gl_context, &program, "position");
            let color_attribute = get_attribute_pointer(gl_context, &program, "a_color");

            FadeBackgroundShader {
                program,
                position_attribute: position_attribute,
                color_attribute: color_attribute,
            }
        };

        let simple_shader = {
            let program = create_program(&gl_context, SIMPLE_VS, SIMPLE_FS)
                .expect("Create Fade Background program");

            let position_attribute = get_attribute_pointer(gl_context, &program, "position");
            let object_transform_uniform =
                get_uniform_location(gl_context, &program, "object_transform");
            let color_uniform = get_uniform_location(gl_context, &program, "color");

            SimpleShader {
                program,
                position_attribute: position_attribute,
                object_transform_uniform,
                color_uniform,
            }
        };

        let billboard_shader = {
            let program = create_program(&gl_context, BILLBOARD_VS, SIMPLE_FS)
                .expect("Create BillBoard Shader");

            let board_position_attribute =
                get_attribute_pointer(gl_context, &program, "board_position");
            let board_center_attribute =
                get_attribute_pointer(gl_context, &program, "board_center");

            log_1(
                &format!(
                    "bpa: {}, bca: {}",
                    board_position_attribute, board_center_attribute
                )
                .into(),
            );
            let color_uniform = get_uniform_location(gl_context, &program, "color");
            let camera_up_uniform = get_uniform_location(gl_context, &program, "camera_up");
            let camera_right_uniform = get_uniform_location(gl_context, &program, "camera_right");
            let worldspace_transform_uniform =
                get_uniform_location(gl_context, &program, "worldspace_transform");

            BillBoardShader {
                program,
                board_position_attribute,
                board_center_attribute,
                color_uniform,
                camera_up_uniform,
                camera_right_uniform,
                worldspace_transform_uniform,
            }
        };

        ShaderSystem {
            fade_background_shader,
            simple_shader,
            billboard_shader,
            active_program: RefCell::new(ShaderKind::Simple),
        }
    }

    pub fn use_program(&self, gl_context: &WebGlRenderingContext, shader_kind: ShaderKind) {
        if *self.active_program.borrow() != shader_kind {
            match shader_kind {
                ShaderKind::Simple => {
                    gl_context.use_program(Some(&self.simple_shader.program));
                }
                ShaderKind::FadeBackground => {
                    gl_context.use_program(Some(&self.fade_background_shader.program));
                }
                ShaderKind::BillBoard => {
                    gl_context.use_program(Some(&self.billboard_shader.program));
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

fn get_attribute_pointer(
    gl_context: &WebGlRenderingContext,
    program: &WebGlProgram,
    attribute_name: &str,
) -> u32 {
    let attribute_signed = gl_context.get_attrib_location(&program, attribute_name);

    if attribute_signed < 0 {
        log_1(&format!("Could not get {} attribute", attribute_name).into());
        panic!("Could not get attribute");
    }

    attribute_signed as u32
}

fn get_uniform_location(
    gl_context: &WebGlRenderingContext,
    program: &WebGlProgram,
    uniform_name: &str,
) -> WebGlUniformLocation {
    let result = gl_context.get_uniform_location(&program, uniform_name);
    match result {
        Some(location) => location,
        None => {
            log_1(&format!("Could not get {} uniform", uniform_name).into());
            panic!("Could not get uniform")
        }
    }
}
