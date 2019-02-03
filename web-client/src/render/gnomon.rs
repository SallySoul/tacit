use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color::*;
use crate::shader::{ShaderKind, ShaderSystem};
use camera::Camera;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;

const FRAME_PROPORTION: f32 = 1.0 / 3.0;
const X_COLOR: Color = RED;
const Y_COLOR: Color = GREEN;
const Z_COLOR: Color = BLUE;

pub struct Gnomon {
    x_vertices: ArrayBuffer,
    x_color: Color,

    y_vertices: ArrayBuffer,
    y_color: Color,

    z_vertices: ArrayBuffer,
    z_color: Color,

    indices: IndexBuffer,
}

impl Gnomon {
    pub fn new(gl_context: &WebGlRenderingContext, width: f32) -> Result<Gnomon, JsValue> {
        let frame_width = FRAME_PROPORTION * width;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let x_vertices_vec = vec![
            0.0, 0.0, 0.0,
            frame_width, 0.0, 0.0,
            width, 0.0, 0.0,
            frame_width, frame_width, 0.0,
            frame_width, 0.0, frame_width,
        ];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let y_vertices_vec = vec![
            0.0, 0.0, 0.0,
            0.0, frame_width, 0.0,
            0.0, width, 0.0,
            frame_width, frame_width, 0.0,
            0.0, frame_width, frame_width,
        ];

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let z_vertices_vec = vec![
            0.0, 0.0, 0.0,
            0.0, 0.0, frame_width,
            0.0, 0.0, width,
            frame_width, 0.0, frame_width,
            0.0, frame_width, frame_width,
        ];

        let indices_vec = vec![0, 1, 1, 2, 1, 3, 1, 4];

        Ok(Gnomon {
            x_vertices: ArrayBuffer::new(gl_context, x_vertices_vec)?,
            x_color: X_COLOR,

            y_vertices: ArrayBuffer::new(gl_context, y_vertices_vec)?,
            y_color: Y_COLOR,

            z_vertices: ArrayBuffer::new(gl_context, z_vertices_vec)?,
            z_color: Z_COLOR,

            indices: IndexBuffer::new(gl_context, indices_vec)?,
        })
    }

    pub fn render(
        &mut self,
        gl_context: &WebGlRenderingContext,
        shader_sys: &ShaderSystem,
        camera: &Camera,
    ) {
        shader_sys.use_program(gl_context, ShaderKind::Simple);

        let object_transform_uniform = &shader_sys.simple_shader.object_transform_uniform;

        let mut object_transform_matrix = camera.get_world_to_clipspace_transform();
        let object_transform_mut_ref: &mut [f32; 16] = object_transform_matrix.as_mut();

        gl_context.uniform_matrix4fv_with_f32_array(
            Some(object_transform_uniform),
            false,
            object_transform_mut_ref.as_mut(),
        );

        let color_uniform = &shader_sys.simple_shader.color_uniform;
        let position_attribute = shader_sys.simple_shader.position_attribute; 

        // Draw_x
        gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut self.x_color);

        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.x_vertices.gl_buffer));
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices.gl_buffer));

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(position_attribute, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(position_attribute);

        gl_context.draw_elements_with_i32(GL::LINES, 8, GL::UNSIGNED_SHORT, 0);

        // Draw_y
        gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut self.y_color);

        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_vertices.gl_buffer));
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices.gl_buffer));

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(position_attribute, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(position_attribute);

        gl_context.draw_elements_with_i32(GL::LINES, 8, GL::UNSIGNED_SHORT, 0);

        // Draw_z
        gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut self.z_color);

        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.z_vertices.gl_buffer));
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices.gl_buffer));

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(position_attribute, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(position_attribute);

        gl_context.draw_elements_with_i32(GL::LINES, 8, GL::UNSIGNED_SHORT, 0);
    }
}
