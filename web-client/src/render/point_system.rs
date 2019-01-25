use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color;
use super::color::Color;
use crate::shader::ShaderSystem;
use smallvec::SmallVec;
use std::mem;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

const DEFAULT_WIDTH: f32 = 1.0;
const DEFAULT_COLOR: Color = color::BLACK;
const MAX_POINTS_PER_BUFFER: usize = (u16::max_value() / 3) as usize;

pub struct PointSystemConstructor<'a> {
    radius: f32,
    color: Color,
    active_float_buffer: Vec<f32>,
    active_point_count: usize,
    filled_float_buffers: SmallVec<[Vec<f32>; 1]>,
    gl_context: &'a WebGlRenderingContext,
    shader_system: &'a ShaderSystem,
}

impl<'a> PointSystemConstructor<'a> {
    pub fn new(
        gl_context: &'a WebGlRenderingContext,
        shader_system: &'a ShaderSystem,
    ) -> PointSystemConstructor<'a> {
        PointSystemConstructor {
            radius: DEFAULT_WIDTH,
            color: DEFAULT_COLOR,
            active_float_buffer: Vec::new(),
            active_point_count: 0,
            filled_float_buffers: SmallVec::new(),
            gl_context,
            shader_system,
        }
    }

    pub fn set_radius(&'a mut self, radius: f32) -> &'a mut PointSystemConstructor {
        self.radius = radius;
        self
    }

    pub fn set_color(&'a mut self, color: Color) -> &'a mut PointSystemConstructor {
        self.color = color;
        self
    }

    pub fn add_point(&'a mut self, position: &[f32; 3]) -> &'a mut PointSystemConstructor {
        if self.active_point_count < MAX_POINTS_PER_BUFFER {
            // Swap out the active (and now full) buffer for an empty one
            let mut full_buffer = Vec::new();
            mem::swap(&mut full_buffer, &mut self.active_float_buffer);

            // Keep track of full buffer, cleanup, and add point
            self.filled_float_buffers.push(full_buffer);
            self.active_point_count = 0;
            self.add_point(position)
        } else {
            self.active_float_buffer.extend(position);
            self.active_point_count += 1;
            self
        }
    }

    pub fn finish(self) -> Result<PointSystem<'a>, JsValue> {
        // Build the instance
        // This geometry should be on the x-y plane,
        // and have (0, 0, 0) as the center
        // TODO: Support arbitrary fidelity instead of quads
        // In the mean time, do lowest quality, quad
        // Vertices:
        // 0: (x, y)
        // 1: (x, -y)
        // 2: (-x, -y)
        // 3: (-x, y)
        // Triangles:
        // 0: (0, 1, 2)
        // 1: (2, 3, 0)
        let quad_side_length = self.radius.sqrt();
        let vertices = vec![
            quad_side_length,
            quad_side_length,
            quad_side_length,
            -quad_side_length,
            -quad_side_length,
            -quad_side_length,
            -quad_side_length,
            quad_side_length,
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        let instance_vertex_buffer = ArrayBuffer::new(&self.gl_context, vertices)?;
        let instance_index_buffer = IndexBuffer::new(&self.gl_context, indices)?;

        // The active buffer, and any filled buffers, need to be sent to GPU
        let mut point_buffers = SmallVec::new();
        point_buffers.push(PointBuffer::new(
            &self.gl_context,
            self.active_float_buffer,
        )?);
        for buffer in self.filled_float_buffers {
            point_buffers.push(PointBuffer::new(&self.gl_context, buffer)?);
        }

        Ok(PointSystem {
            color: self.color,
            instance_vertex_buffer,
            instance_index_buffer,
            point_buffers,
            gl_context: self.gl_context,
            shader_system: self.shader_system,
        })
    }
}

pub struct PointBuffer {
    pub position_buffer: ArrayBuffer,
    pub index_buffer: IndexBuffer,
}

impl PointBuffer {
    fn new(
        gl_context: &WebGlRenderingContext,
        float_buffer: Vec<f32>,
    ) -> Result<PointBuffer, JsValue> {
        let point_count = float_buffer.len() as u16 / 3;

        Ok(PointBuffer {
            position_buffer: ArrayBuffer::new(gl_context, float_buffer)?,
            index_buffer: IndexBuffer::new(gl_context, (0..point_count).collect())?,
        })
    }
}

pub struct PointSystem<'a> {
    color: Color,
    instance_vertex_buffer: ArrayBuffer,
    instance_index_buffer: IndexBuffer,
    point_buffers: SmallVec<[PointBuffer; 1]>,
    gl_context: &'a WebGlRenderingContext,
    shader_system: &'a ShaderSystem,
}

impl<'a> PointSystem<'a> {
    fn render(&self) {
        // bind color uniform

        // bind instance buffers

        // foreach point_buffer
        //for point_buffer in self.point_buffers {
        // bind point_buffers

        // instanced draw_call
        //}
    }
}
