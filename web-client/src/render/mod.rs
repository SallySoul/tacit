use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

use js_sys::{Float32Array, Uint16Array};

use crate::shader::{ShaderKind, ShaderSystem};
use camera::Camera;
use geoprim::Plot;
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
    line_count: i32,
    point_vertices_float_buffer: ArrayBuffer,
    line_vertices_float_buffer: ArrayBuffer,
    line_indices_buffer: IndexBuffer,
}

impl PlotBuffers {
    fn new(gl_context: &WebGlRenderingContext, plot: &Plot) -> Result<PlotBuffers, JsValue> {
        // Extract the coords from each point, and create an array buffer
        let mut point_vertices_float_vec = Vec::with_capacity(plot.points.len() * 3);
        for point in &plot.points {
            point_vertices_float_vec.push(point.x);
            point_vertices_float_vec.push(point.y);
            point_vertices_float_vec.push(point.z);
        }
        let point_vertices_float_buffer = ArrayBuffer::new(gl_context, point_vertices_float_vec)?;

        // Extract the coords from each point in each line, and create an array buffer
        let mut line_vertices_float_vec = Vec::with_capacity(plot.points.len() * 3);
        for line in &plot.lines {
            for point in &[line.p1, line.p2] {
                line_vertices_float_vec.push(point.x);
                line_vertices_float_vec.push(point.y);
                line_vertices_float_vec.push(point.z);
            }
        }
        let line_vertices_float_buffer = ArrayBuffer::new(gl_context, line_vertices_float_vec)?;

        // Line Indices in this case are sequential. Inefficient...
        let line_index_count = plot.lines.len() * 2;
        let mut line_indices_vec = Vec::with_capacity(line_index_count);
        line_indices_vec.extend(0..line_index_count as u16);
        let line_indices_buffer = IndexBuffer::new(gl_context, line_indices_vec)?;

        Ok(PlotBuffers {
            point_count: plot.points.len() as i32,
            line_count: plot.lines.len() as i32,
            point_vertices_float_buffer,
            line_vertices_float_buffer,
            line_indices_buffer,
        })
    }
}

pub type WebRendererWrapper = Rc<RefCell<WebRenderer>>;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    plot_buffers: Option<PlotBuffers>,
    pub gl_context: WebGlRenderingContext,
}

impl WebRenderer {
    pub fn new_wrapper(gl_context: WebGlRenderingContext) -> WebRendererWrapper {
        let shader_sys = ShaderSystem::new(&gl_context);

        Rc::new(RefCell::new(WebRenderer {
            shader_sys,
            plot_buffers: None,
            gl_context,
        }))
    }

    pub fn set_plot(&mut self, plot: &Plot) -> Result<(), JsValue> {
        log_1(&"Set_plot in renderer".into());
        let plot_buffers = PlotBuffers::new(&self.gl_context, plot)?;

        log_1(
            &format!(
                "line_v_f_b len: {}",
                plot_buffers.line_vertices_float_buffer.float_array.length()
            )
            .into(),
        );
        log_1(&format!("lines: {}", plot.lines.len()).into());
        self.plot_buffers = Some(plot_buffers);
        Ok(())
    }

    pub fn clear_plot(&mut self) {
        self.plot_buffers = None;
    }

    pub fn render(&self, camera: &Camera) {
        // Parchment color?
        self.gl_context.clear_color(0.952, 0.885, 0.792, 1.);
        self.gl_context
            .clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let width = self.gl_context.drawing_buffer_width();
        let height = self.gl_context.drawing_buffer_height();

        self.gl_context.viewport(0, 0, width, height);

        //self.gl_context.line_width(4.0);

        match &self.plot_buffers {
            Some(plot_buffers) => self.render_plot_buffers(plot_buffers, camera),
            None => (),
        };
    }

    fn render_plot_buffers(&self, plot_buffers: &PlotBuffers, camera: &Camera) {
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

        let color_uniform =
            shader.get_uniform_location(&self.gl_context, "color");

        let mut line_color = color::WHITE;
        self.gl_context.uniform4fv_with_f32_array(
            color_uniform.as_ref(),
            &mut line_color
        );

        // Bind buffers
        self.gl_context.bind_buffer(
            GL::ARRAY_BUFFER,
            Some(&plot_buffers.line_vertices_float_buffer.gl_buffer),
        );
        self.gl_context.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&plot_buffers.line_indices_buffer.gl_buffer),
        );

        // Get the attribute location
        let position_attribute = self
            .gl_context
            .get_attrib_location(&shader.program, "position");

        // Point an attribute to the currently bound VBO
        self.gl_context
            .vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        // Enable the attribute
        self.gl_context.enable_vertex_attrib_array(0);

        self.gl_context.draw_elements_with_i32(
            GL::LINES,
            plot_buffers.line_count * 2,
            GL::UNSIGNED_SHORT,
            0,
        );

        // Unbind buffers
        self.gl_context.bind_buffer(GL::ARRAY_BUFFER, None);
        self.gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
    }
}
