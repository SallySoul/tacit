use implicit_mesh::cell_keys::morton_keys::MortonKey;
use implicit_mesh::function_ir::Node;
use implicit_mesh::mesh_tree::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use js_sys::{Float32Array, Uint16Array};

use crate::shader::{Shader, ShaderKind, ShaderSystem};
use camera::Camera;
use std::cell::RefCell;
use std::rc::Rc;

mod color;

use color::*;

// The key is that we maintain a rust vec for ease, but package a Float32Array "view" into said vec
struct ArrayBuffer {
    float_buffer: Vec<f32>,
    float_array: Float32Array,
    gl_buffer: WebGlBuffer,
}

impl ArrayBuffer {
    fn new(
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

struct IndexBuffer {
    u16_buffer: Vec<u16>,
    u16_array: Uint16Array,
    gl_buffer: WebGlBuffer,
}

impl IndexBuffer {
    fn new(
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

struct PlotBuffers {
    point_count: i32,
    point_vertices_buffer: ArrayBuffer,
    point_indices_buffer: IndexBuffer,

    edge_count: i32,
    edge_vertices_buffer: ArrayBuffer,
    edge_indices_buffer: IndexBuffer,

    bb_edge_count: i32,
    bb_vertices_buffer: ArrayBuffer,
    bb_indices_buffer: IndexBuffer,
}

impl PlotBuffers {
    fn new(
        gl_context: &WebGlRenderingContext,
        mtree: &MeshTree<MortonKey, Node>,
    ) -> Result<PlotBuffers, JsValue> {
        let point_float_vec = mtree.get_vertex_floats();
        let point_count = point_float_vec.len() / 3;
        let point_vertices_buffer = ArrayBuffer::new(gl_context, point_float_vec)?;
        let point_indices_buffer = IndexBuffer::new(gl_context, (0..point_count as u16).collect())?;

        let edge_float_vec = mtree.get_edge_floats();
        let edge_vertex_count = edge_float_vec.len() / 3;
        let edge_vertices_buffer = ArrayBuffer::new(gl_context, edge_float_vec)?;
        let edge_indices_buffer =
            IndexBuffer::new(gl_context, (0..edge_vertex_count as u16).collect())?;

        let bb_float_vec = mtree.get_bounding_box_floats();
        let bb_vertex_count = bb_float_vec.len() / 3;
        let bb_vertices_buffer = ArrayBuffer::new(gl_context, bb_float_vec)?;
        let bb_indices_buffer =
            IndexBuffer::new(gl_context, (0..bb_vertex_count as u16).collect())?;

        Ok(PlotBuffers {
            point_count: point_count as i32,
            point_vertices_buffer,
            point_indices_buffer,
            edge_count: (edge_vertex_count / 2) as i32,
            edge_vertices_buffer,
            edge_indices_buffer,
            bb_edge_count: (bb_vertex_count / 2) as i32,
            bb_vertices_buffer,
            bb_indices_buffer,
        })
    }

    fn render_edges(&self, gl_context: &WebGlRenderingContext, shader: &Shader) {
        let color_uniform = shader.get_uniform_location(&gl_context, "color");

        let mut edge_color = Color::from_floats(0.2, 0.33, 0.84, 1.0);
        gl_context.uniform4fv_with_f32_array(color_uniform.as_ref(), &mut edge_color);

        // Bind buffers
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.edge_vertices_buffer.gl_buffer));
        gl_context.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&self.edge_indices_buffer.gl_buffer),
        );

        // Get the attribute location
        let position_attribute = gl_context.get_attrib_location(&shader.program, "position");

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(0);

        gl_context.draw_elements_with_i32(GL::LINES, self.edge_count * 2, GL::UNSIGNED_SHORT, 0);
    }

    fn render_bb(&self, gl_context: &WebGlRenderingContext, shader: &Shader) {
        let color_uniform = shader.get_uniform_location(&gl_context, "color");

        let mut edge_color = Color::from_floats(0.68, 0.04, 0.23, 1.0);
        gl_context.uniform4fv_with_f32_array(color_uniform.as_ref(), &mut edge_color);

        // Bind buffers
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.bb_vertices_buffer.gl_buffer));
        gl_context.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&self.bb_indices_buffer.gl_buffer),
        );

        // Get the attribute location
        let position_attribute = gl_context.get_attrib_location(&shader.program, "position");

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(0);

        gl_context.draw_elements_with_i32(GL::LINES, self.bb_edge_count * 2, GL::UNSIGNED_SHORT, 0);
    }

    fn render_points(&self, gl_context: &WebGlRenderingContext, shader: &Shader) {
        let color_uniform = shader.get_uniform_location(&gl_context, "color");

        let mut edge_color = Color::from_floats(0.33, 0.86, 0.42, 1.0);
        gl_context.uniform4fv_with_f32_array(color_uniform.as_ref(), &mut edge_color);

        // Bind buffers
        gl_context.bind_buffer(
            GL::ARRAY_BUFFER,
            Some(&self.point_vertices_buffer.gl_buffer),
        );
        gl_context.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&self.point_indices_buffer.gl_buffer),
        );

        // Get the attribute location
        let position_attribute = gl_context.get_attrib_location(&shader.program, "position");

        // Point an attribute to the currently bound VBO
        gl_context.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        gl_context.enable_vertex_attrib_array(0);

        gl_context.draw_elements_with_i32(GL::POINTS, self.point_count, GL::UNSIGNED_SHORT, 0);
    }
}

pub type WebRendererWrapper = Rc<RefCell<WebRenderer>>;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    plot_buffers: Option<PlotBuffers>,
    pub gl_context: WebGlRenderingContext,
    draw_vertices: bool,
    draw_edges: bool,
    draw_bb: bool,
}

impl WebRenderer {
    pub fn new_wrapper(gl_context: WebGlRenderingContext) -> WebRendererWrapper {
        let shader_sys = ShaderSystem::new(&gl_context);

        Rc::new(RefCell::new(WebRenderer {
            shader_sys,
            plot_buffers: None,
            gl_context,
            draw_vertices: crate::DRAW_VERTICES_START,
            draw_edges: crate::DRAW_EDGES_START,
            draw_bb: crate::DRAW_BB_START,
        }))
    }

    pub fn set_draw_vertices(&mut self, draw_flag: bool) {
        self.draw_vertices = draw_flag;
    }

    pub fn set_draw_edges(&mut self, draw_flag: bool) {
        self.draw_edges = draw_flag;
    }

    pub fn set_draw_bb(&mut self, draw_flag: bool) {
        self.draw_bb = draw_flag;
    }

    pub fn set_plot(&mut self, mtree: &MeshTree<MortonKey, Node>) -> Result<(), JsValue> {
        log_1(&"Set_plot in renderer".into());
        let plot_buffers = PlotBuffers::new(&self.gl_context, mtree)?;
        self.plot_buffers = Some(plot_buffers);
        Ok(())
    }

    pub fn clear_plot(&mut self) {
        self.plot_buffers = None;
    }

    pub fn render(&self, camera: &Camera) {
        // Parchment color?
        self.gl_context.clear_color(0.13, 0.11, 0.21, 1.);
        self.gl_context
            .clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let width = self.gl_context.drawing_buffer_width();
        let height = self.gl_context.drawing_buffer_height();

        self.gl_context.viewport(0, 0, width, height);

        //self.gl_context.line_width(4.0);
        let shader = self.shader_sys.get_shader(&ShaderKind::Simple).unwrap();

        let object_transform_uniform =
            shader.get_uniform_location(&self.gl_context, "object_transform");

        let mut object_transform_matrix = camera.get_world_to_clipspace_transform();
        let object_transform_mut_ref: &mut [f32; 16] = object_transform_matrix.as_mut();
        self.gl_context.uniform_matrix4fv_with_f32_array(
            object_transform_uniform.as_ref(),
            false,
            object_transform_mut_ref.as_mut(),
        );

        match &self.plot_buffers {
            Some(plot_buffers) => {
                if self.draw_edges {
                    plot_buffers.render_edges(&self.gl_context, shader);
                }
                if self.draw_bb {
                    plot_buffers.render_bb(&self.gl_context, shader);
                }
                if self.draw_vertices {
                    plot_buffers.render_points(&self.gl_context, shader);
                }
            }
            None => (),
        };
    }
}
