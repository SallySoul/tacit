use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color::*;
use crate::shader::{ShaderKind, ShaderSystem};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;

pub struct FadeBackground {
    vertices: ArrayBuffer,
    colors: ArrayBuffer,
    indices: IndexBuffer,
}

impl FadeBackground {
    pub fn new(gl_context: &WebGlRenderingContext) -> Result<FadeBackground, JsValue> {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertex_vec = vec! [
            -1.0, 1.0,  0.0,
            1.0,  1.0,  0.0,
            1.0,  -1.0, 0.0,
            -1.0, -1.0, 0.0,
        ];

        // Each vertex gets a color, which has four components
        let mut color_vec = Vec::with_capacity(16);
        color_vec.extend_from_slice(&WHITE);
        color_vec.extend_from_slice(&WHITE);
        color_vec.extend_from_slice(&GREY);
        color_vec.extend_from_slice(&GREY);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices_vec = vec![
            0, 1, 2,
            2, 3, 0,
        ];

        Ok(FadeBackground {
            vertices: ArrayBuffer::new(gl_context, vertex_vec)?,
            colors: ArrayBuffer::new(gl_context, color_vec)?,
            indices: IndexBuffer::new(gl_context, indices_vec)?,
        })
    }

    pub fn render(&self, gl_context: &WebGlRenderingContext, shader_system: &ShaderSystem) {
        let shader = shader_system.use_program(gl_context, ShaderKind::FadeBackground);

        // Setup the postion attribute
        let position_attribute = gl_context.get_attrib_location(&shader.program, "position") as u32;
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertices.gl_buffer));
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices.gl_buffer));
        gl_context.vertex_attrib_pointer_with_i32(position_attribute, 3, GL::FLOAT, false, 0, 0);
        gl_context.enable_vertex_attrib_array(position_attribute);

        // Setup the color attribute
        let color_attribute_signed = gl_context.get_attrib_location(&shader.program, "a_color");

        if color_attribute_signed < 0 {
            panic!("Can't get a_color attribute");
        }

        let color_attribute = color_attribute_signed as u32;

        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.colors.gl_buffer));
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices.gl_buffer));
        gl_context.vertex_attrib_pointer_with_i32(color_attribute, 4, GL::FLOAT, false, 0, 0);
        gl_context.enable_vertex_attrib_array(color_attribute);
        gl_context.draw_elements_with_i32(GL::TRIANGLES, 6, GL::UNSIGNED_SHORT, 0);
    }
}
