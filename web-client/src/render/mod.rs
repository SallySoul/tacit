use implicit_mesh::cell_keys::morton_keys::MortonKey;
use implicit_mesh::function_ir::Node;
use implicit_mesh::mesh_tree::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlRenderingContext;

use crate::shader::ShaderSystem;
use camera::Camera;
use std::cell::RefCell;
use std::rc::Rc;

mod buffers;
mod color;
mod fade_background;
mod gnomon;
mod plot_buffers;
use plot_buffers::PlotBuffers;

pub type WebRendererWrapper = Rc<RefCell<WebRenderer>>;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    plot_buffers: Option<PlotBuffers>,
    pub gl_context: WebGlRenderingContext,
    draw_vertices: bool,
    draw_edges: bool,
    draw_bb: bool,
    draw_gnomon_center: bool,
    draw_gnomon_corner: bool,
    gnomon: gnomon::Gnomon,
    corner_gnomon: gnomon::Gnomon,
    fade_background: fade_background::FadeBackground,
}

impl WebRenderer {
    pub fn new_wrapper(gl_context: WebGlRenderingContext) -> Result<WebRendererWrapper, JsValue> {
        let shader_sys = ShaderSystem::new(&gl_context);
        let gnomon = gnomon::Gnomon::new(&gl_context, 20.0)?;
        let corner_gnomon = gnomon::Gnomon::new(&gl_context, 1.0)?;
        let fade_background = fade_background::FadeBackground::new(&gl_context)?;

        Ok(Rc::new(RefCell::new(WebRenderer {
            shader_sys,
            plot_buffers: None,
            gl_context,
            draw_vertices: crate::DRAW_VERTICES_START,
            draw_edges: crate::DRAW_EDGES_START,
            draw_bb: crate::DRAW_BB_START,
            draw_gnomon_center: crate::DRAW_GNOMON_CENTER_START,
            draw_gnomon_corner: crate::DRAW_GNOMON_CORNER_START,
            gnomon,
            corner_gnomon,
            fade_background,
        })))
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

    pub fn set_draw_gnomon_center(&mut self, draw_flag: bool) {
        self.draw_gnomon_center = draw_flag;
    }

    pub fn set_draw_gnomon_corner(&mut self, draw_flag: bool) {
        self.draw_gnomon_corner = draw_flag;
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

    pub fn render(&mut self, camera: &Camera) {
        // Parchment color?
        self.gl_context.clear_color(0.13, 0.11, 0.21, 1.);
        self.gl_context
            .clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let width = self.gl_context.drawing_buffer_width();
        let height = self.gl_context.drawing_buffer_height();
        self.gl_context.viewport(0, 0, width, height);

        self.gl_context.disable(GL::DEPTH_TEST);
        self.fade_background
            .render(&self.gl_context, &self.shader_sys);
        self.gl_context.enable(GL::DEPTH_TEST);

        match &self.plot_buffers {
            Some(plot_buffers) => {
                plot_buffers.render(
                    &self.gl_context,
                    &self.shader_sys,
                    camera,
                    self.draw_edges,
                    self.draw_bb,
                    self.draw_vertices,
                );
            }
            None => (),
        };

        if self.draw_gnomon_center {
            self.gnomon
                .render(&self.gl_context, &self.shader_sys, camera, false);
        }

        if self.draw_gnomon_corner {
            self.corner_gnomon
                .render(&self.gl_context, &self.shader_sys, camera, true);
        }
    }
}
