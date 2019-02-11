use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color;
use super::color::Color;
use crate::shader::{ShaderKind, ShaderSystem};
use camera::Camera;
use std::mem;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlRenderingContext;

const DEFAULT_WIDTH: f32 = 1.0;
const DEFAULT_COLOR: Color = color::BLACK;
const MAX_POINTS_PER_BUFFER: usize = (u16::max_value() / 20) as usize;

pub struct PointSystemBuilder<'a> {
    radius: f32,
    color: Color,
    active_float_buffer: Vec<f32>,
    active_point_count: usize,
    filled_float_buffers: Vec<Vec<f32>>,
    gl_context: &'a WebGlRenderingContext,
}

impl<'a> PointSystemBuilder<'a> {
    pub fn new(gl_context: &'a WebGlRenderingContext) -> PointSystemBuilder<'a> {
        PointSystemBuilder {
            radius: DEFAULT_WIDTH,
            color: DEFAULT_COLOR,
            active_float_buffer: Vec::new(),
            active_point_count: 0,
            filled_float_buffers: Vec::new(),
            gl_context,
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn add_point(&mut self, position: [f32; 3]) {
        if self.active_point_count < MAX_POINTS_PER_BUFFER {
            // We have four vertices per bill point bill board
            // Without instancing, we need to have a board center for each
            self.active_float_buffer.extend_from_slice(&position);
            self.active_float_buffer.extend_from_slice(&position);
            self.active_float_buffer.extend_from_slice(&position);
            self.active_float_buffer.extend_from_slice(&position);
            self.active_point_count += 1;
        } else {
            // Swap out the active (and now full) buffer for an empty one
            let mut full_buffer = Vec::new();
            mem::swap(&mut full_buffer, &mut self.active_float_buffer);

            // Keep track of full buffer, cleanup, and add point
            self.filled_float_buffers.push(full_buffer);
            self.active_point_count = 0;
            self.add_point(position)
        }
    }

    pub fn finish(mut self) -> Result<PointSystem, JsValue> {
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

        self.filled_float_buffers.push(self.active_float_buffer);
        let max_buffer_size = &self
            .filled_float_buffers
            .iter()
            .map(|buffer| buffer.len())
            .max()
            .unwrap();

        let mut vertices = Vec::with_capacity(8 * max_buffer_size);
        let mut indices = Vec::with_capacity(6 * max_buffer_size);
        for i in 0..*max_buffer_size / 12 {
            vertices.extend_from_slice(&[
                quad_side_length,
                quad_side_length,
                quad_side_length,
                -quad_side_length,
                -quad_side_length,
                -quad_side_length,
                -quad_side_length,
                quad_side_length,
            ]);
            let j = i * 4;
            indices.extend_from_slice(&[
                0 + j as u16,
                1 + j as u16,
                2 + j as u16,
                2 + j as u16,
                3 + j as u16,
                0 + j as u16,
            ]);
        }
        let instance_vertices = ArrayBuffer::new(&self.gl_context, vertices)?;
        let instance_indices = IndexBuffer::new(&self.gl_context, indices)?;

        // The active buffer, and any filled buffers, need to be sent to GPU
        let mut point_buffers = Vec::new();
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
    pub vertices: ArrayBuffer,
    pub size: i32,
}

impl PointBuffer {
    fn new(
        gl_context: &WebGlRenderingContext,
        float_buffer: Vec<f32>,
    ) -> Result<PointBuffer, JsValue> {
        let size = float_buffer.len() as i32 / 3;

        Ok(PointBuffer {
            vertices: ArrayBuffer::new(gl_context, float_buffer)?,
            size,
        })
    }
}

pub struct PointSystem {
    color: Color,
    instance_vertices: ArrayBuffer,
    instance_indices: IndexBuffer,
    point_buffers: Vec<PointBuffer>,
}

impl PointSystem {
    pub fn render(
        &self,
        gl_context: &WebGlRenderingContext,
        shader_sys: &ShaderSystem,
        camera: &Camera,
    ) {
        // Setup GL State
        shader_sys.use_program(gl_context, ShaderKind::BillBoard);
        let width = gl_context.drawing_buffer_width();
        let height = gl_context.drawing_buffer_height();
        gl_context.viewport(0, 0, width, height);

        // Bind uniforms
        let camera_up_uniform = &shader_sys.billboard_shader.camera_up_uniform;
        let mut camera_up = camera.get_up();
        let camera_up_mut_ref: &mut [f32; 3] = camera_up.as_mut();
        gl_context.uniform3fv_with_f32_array(Some(camera_up_uniform), camera_up_mut_ref.as_mut());

        let camera_right_uniform = &shader_sys.billboard_shader.camera_right_uniform;
        let mut camera_right = camera.get_right();
        let camera_right_mut_ref: &mut [f32; 3] = camera_right.as_mut();
        gl_context
            .uniform3fv_with_f32_array(Some(camera_right_uniform), camera_right_mut_ref.as_mut());

        let worldspace_transform_uniform =
            &shader_sys.billboard_shader.worldspace_transform_uniform;
        let mut worldspace_transform = camera.get_world_to_clipspace_transform();
        let worldspace_transform_mut_ref: &mut [f32; 16] = worldspace_transform.as_mut();
        gl_context.uniform_matrix4fv_with_f32_array(
            Some(worldspace_transform_uniform),
            false,
            worldspace_transform_mut_ref.as_mut(),
        );

        let color_uniform = &shader_sys.billboard_shader.color_uniform;
        gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut self.color.clone());

        // Bind board geometry buffers
        let board_position_attribute = shader_sys.billboard_shader.board_position_attribute;
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.instance_vertices.gl_buffer));
        gl_context.vertex_attrib_pointer_with_i32(
            board_position_attribute,
            2,
            GL::FLOAT,
            false,
            0,
            0,
        );
        gl_context.enable_vertex_attrib_array(board_position_attribute);

        gl_context.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&self.instance_indices.gl_buffer),
        );

        for point_buffer in &self.point_buffers {
            let vertex_count = point_buffer.size;

            log_1(
                &format!(
                    "bpa size: {}, bca size: {}, indices size: {} vertex_count: {}",
                    self.instance_vertices.float_buffer.len(),
                    point_buffer.vertices.float_buffer.len(),
                    self.instance_indices.u16_buffer.len(),
                    vertex_count
                )
                .into(),
            );

            // Bind board center buffers
            let board_center_attribute = shader_sys.billboard_shader.board_center_attribute;
            gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&point_buffer.vertices.gl_buffer));
            gl_context.vertex_attrib_pointer_with_i32(
                board_center_attribute,
                3,
                GL::FLOAT,
                false,
                0,
                0,
            );
            gl_context.enable_vertex_attrib_array(board_center_attribute);

            //draw
            gl_context.draw_elements_with_i32(GL::TRIANGLES, vertex_count, GL::UNSIGNED_SHORT, 0);
        }
    }
}
