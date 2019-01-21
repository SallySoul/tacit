use crate::render::WebRendererWrapper;
use camera::{ButtonState, Camera, MouseButton};
use geoprim::Plot;
use implicit_mesh::function_ir::Node;
use implicit_mesh::interval::Interval;
use implicit_mesh::mesh_tree::*;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;

pub type AppWrapper = Rc<RefCell<App>>;

pub struct App {
    equation: String,
    pub camera: Camera,
    renderer: Option<WebRendererWrapper>,
    mtree: Option<MeshTree<implicit_mesh::key::MortonKey, Node>>,
    draw_bb: bool,
    draw_vertices: bool,
    draw_edges: bool,
}

impl App {
    pub fn new_wrapper() -> AppWrapper {
        Rc::new(RefCell::new(App {
            equation: format!("x + y"),
            camera: Camera::new(),
            renderer: None,
            mtree: None,
            draw_bb: crate::DRAW_BB_START,
            draw_vertices: crate::DRAW_VERTICES_START,
            draw_edges: crate::DRAW_EDGES_START,
        }))
    }

    pub fn set_renderer(&mut self, renderer: WebRendererWrapper) {
        self.renderer = Some(renderer);
    }

    pub fn update_plot(&mut self) {
        let mut plot = Plot::new();
        if let Some(mtree) = &mut self.mtree {
            mtree.add_to_plot(
                self.draw_bb,
                self.draw_vertices,
                self.draw_edges,
                false,
                &mut plot,
            );
        } else {
            return;
        }

        if let Some(renderer) = &self.renderer {
            renderer
                .borrow_mut()
                .set_plot(&plot)
                .expect("Unable to set_plot for renderer");
        }
    }

    pub fn handle_message(&mut self, message: &Message) {
        match message {
            Message::MouseDown(x, y) => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Pressed);
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            }
            Message::MouseUp => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Released);
            }
            Message::MouseMove(x, y) => {
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            }
            Message::Zoom(delta) => {
                self.camera.handle_scroll(*delta);
            }
            Message::EnterEquation(equation) => {
                self.equation.clear();
                self.equation += equation;
                let input: Vec<char> = equation.chars().collect();

                let a = implicit_mesh::parser::parse_expression(&input, 0).expect("not parseable");

                let size_interval = Interval::new(-40.0 / 2.0, 40.0 / 2.0);
                let bounding_box = BoundingBox {
                    x: size_interval.clone(),
                    y: size_interval.clone(),
                    z: size_interval.clone(),
                };

                self.mtree = Some(MeshTree::new(a, bounding_box));

                self.update_plot();
                if let Some(mtree) = &self.mtree {
                    log_1(
                        &format!(
                            "App: Made mtree, level: {}, solution cell count: {}",
                            mtree.get_level(),
                            mtree.get_solution_cell_count()
                        )
                        .into(),
                    );
                }
            }
            Message::Clear => {
                self.equation.clear();
                self.mtree = None;

                match &mut self.renderer {
                    Some(renderer) => renderer.borrow_mut().clear_plot(),
                    None => (),
                }
            }
            Message::Update(time_delta, window_width, window_height) => {
                self.camera
                    .update(*time_delta, *window_width, *window_height);
            }
            Message::NextLevel => {
                let mut plot = Plot::new();
                match &mut self.mtree {
                    Some(mtree) => {
                        mtree.next_level();
                        mtree.generate_edge_set();
                        mtree.generate_vertex_map();
                        mtree.generate_triangle_set();

                        log_1(
                            &format!(
                                "App: level: {}, solution cell count: {}",
                                mtree.get_level(),
                                mtree.get_solution_cell_count()
                            )
                            .into(),
                        );
                    }
                    None => {
                        log_1(&"App: no mtree to next level".into());
                        return;
                    }
                };
                self.update_plot();
            }
            Message::Relax => {
                match &mut self.mtree {
                    Some(mtree) => {
                        mtree.relax_vertices();
                    }
                    None => {
                        return;
                    }
                };
                self.update_plot();
            }
            Message::DrawBoundingBoxes(draw_flag) => {
                log_1(&format!("App: draw bb: {}", draw_flag).into());
                self.draw_bb = *draw_flag;
                self.update_plot();
            }
            Message::DrawVertices(draw_flag) => {
                log_1(&format!("App: draw vertices: {}", draw_flag).into());
                self.draw_vertices = *draw_flag;
                self.update_plot();
            }
            Message::DrawEdges(draw_flag) => {
                log_1(&format!("App: draw edges: {}", draw_flag).into());
                self.draw_edges = *draw_flag;
                self.update_plot();
            }
        }
    }
}

pub enum Message {
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    Zoom(f32),
    EnterEquation(String),
    Update(f32, f32, f32),
    NextLevel,
    Relax,
    Clear,
    DrawBoundingBoxes(bool),
    DrawVertices(bool),
    DrawEdges(bool),
}
