use js_sys::{Float32Array, Uint16Array};
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

// The key is that we maintain a rust vec for ease, but package a Float32Array "view" into said vec
#[allow(dead_code)]
pub struct ArrayBuffer {
    pub float_buffer: Vec<f32>,
    float_array: Float32Array,
    pub gl_buffer: WebGlBuffer,
}

impl ArrayBuffer {
    pub fn new(
        gl_context: &WebGlRenderingContext,
        float_buffer: Vec<f32>,
    ) -> Result<ArrayBuffer, JsValue> {
        let float_array;
        unsafe {
            float_array = Float32Array::view(&float_buffer);
        }

        let gl_buffer = gl_context
            .create_buffer()
            .ok_or("Failed to create vertex buffer")?;

        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&gl_buffer));

        gl_context.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &float_array,
            GL::STATIC_DRAW,
        );

        // Unbind the buffer to be safe
        gl_context.bind_buffer(GL::ARRAY_BUFFER, None);

        Ok(ArrayBuffer {
            float_buffer,
            float_array,
            gl_buffer,
        })
    }
}

#[allow(dead_code)]
pub struct IndexBuffer {
    pub u16_buffer: Vec<u16>,
    u16_array: Uint16Array,
    pub gl_buffer: WebGlBuffer,
}

impl IndexBuffer {
    pub fn new(
        gl_context: &WebGlRenderingContext,
        u16_buffer: Vec<u16>,
    ) -> Result<IndexBuffer, JsValue> {
        let u16_array;
        unsafe {
            u16_array = Uint16Array::view(&u16_buffer);
        }

        let gl_buffer = gl_context
            .create_buffer()
            .ok_or("Failed to create index buffer")?;

        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&gl_buffer));

        gl_context.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &u16_array,
            GL::STATIC_DRAW,
        );

        // Unbind the buffer to be safe
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);

        Ok(IndexBuffer {
            u16_buffer,
            u16_array,
            gl_buffer,
        })
    }
}
