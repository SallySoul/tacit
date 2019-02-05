use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color;
use super::color::Color;
use crate::shader::{ShaderSystem, ShaderKind};
use smallvec::SmallVec;
use std::mem;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlRenderingContext;
use camera::Camera;

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

    pub fn finish(self) -> Result<PointSystem, JsValue> {
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

        #[cfg_attr(rustfmt, rustfmt_skip)]
        let vertices = vec![
            quad_side_length,   quad_side_length,
            quad_side_length,  -quad_side_length,
            -quad_side_length, -quad_side_length,
            -quad_side_length,  quad_side_length,
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];
        let instance_vertices = ArrayBuffer::new(&self.gl_context, vertices)?;
        let instance_indices = IndexBuffer::new(&self.gl_context, indices)?;

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
            instance_vertices,
            instance_indices,
            point_buffers,
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

pub struct PointSystem {
    color: Color,
    instance_vertices: ArrayBuffer,
    instance_indices: IndexBuffer,
    point_buffers: SmallVec<[PointBuffer; 1]>,
}

impl PointSystem {
    pub fn render(
        &mut self,
        gl_context: &WebGlRenderingContext,
        shader_sys: &ShaderSystem,
        camera: &Camera,
    ) {
        // Setup GL State
        shader_sys.use_program(ShaderKind::BillBoard);
        let width = gl_context.drawing_buffer_width();
        let height = gl_context.drawing_buffer_height();
        gl_context.viewport(0, 0, width, height);

        // Bind uniforms
        let camera_up_uniform = shader_sys.billboard_shader.camera_up_uniform;
        let mut camera_up = camera.get_up();
        let camera_up_mut_ref: &mut [f32; 4] = camera_up.as_mut();
        gl_context.uniform_vec3_with_f32_array(
            Some(camera_up_uniform),
            false,
            camera_up_mut_ref.as_mut()
        );
        
        let camera_right_uniform = shader_sys.billboard_shader.camera_right_uniform;
        let mut camera_right = camera.get_right();
        let camera_right_mut_ref: &mut [f32; 4] = camera_right.as_mut();
        gl_context.uniform_vec3_with_f32_array(
            Some(camera_right_uniform),
            false,
            camera_right_mut_ref.as_mut()
        );

        let worldspace_transform_uniform = shader_sys.billboard_shader.worldspace_transform_uniform;
        let worldspace_transform = camera.get_world_to_clipspace_transform();
        let worldspace_transform_mut_ref: &mut [f32; 16] = worldspace_transform.as_mut();
        gl_context.uniform4fv_with_f32_array(
            Some(worldspace_transform_uniform),
            false,
            worldspace_transform_mut_ref.as_mut()
        );

        let color_uniform = shader_sys.billboard_shader.color_attribute;
        gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut self.color);

        // Bind instance buffers
        let board_position_attribute = shader_sys.billboard_shader.board_position_attribute;
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(self.instance_vertices.gl_buffer));
        gl_context.bind_buffer(GL::INDEX_BUFFER, Some(self.instance_indices.gl_buffer));
        gl_context.vertex_attrib_pointer_with_i32(board_position_attribute, 2, GL::FLOAT, false, 0, 0);
        gl_context.enable_vertex_attrib_array(board_position_attribute);
    }
}
