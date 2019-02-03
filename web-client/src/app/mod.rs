use crate::render::WebRendererWrapper;
use camera::{ButtonState, Camera, MouseButton};
use cgmath::{InnerSpace};
use implicit_mesh::cell_keys::morton_keys::MortonKey;
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
    mtree: Option<MeshTree<MortonKey, Node>>,
}

impl App {
    pub fn new_wrapper() -> AppWrapper {
        Rc::new(RefCell::new(App {
            equation: format!("x + y"),
            camera: Camera::new(),
            renderer: None,
            mtree: None,
        }))
    }

    pub fn set_renderer(&mut self, renderer: WebRendererWrapper) {
        self.renderer = Some(renderer);
    }

    pub fn update_plot(&mut self) {
        if let (Some(mtree), Some(renderer)) = (&mut self.mtree, &self.renderer) {
            renderer
                .borrow_mut()
                .set_plot(&mtree)
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
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_bb(*draw_flag);
                }
                self.update_plot();
            }
            Message::DrawVertices(draw_flag) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_vertices(*draw_flag);
                }
                self.update_plot();
            }
            Message::DrawEdges(draw_flag) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_edges(*draw_flag);
                }
                self.update_plot();
            }
            Message::DrawGnomon(draw_flag) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.borrow_mut().set_draw_gnomon(*draw_flag);
                }
                self.update_plot();
            }
            Message::DefaultCam => {
                self.camera.transition_to_default();
            }
            Message::Debug => {
                let up = self.camera.get_up();
                let right = self.camera.get_right();

                log_1(&format!("up: {:?}, norm: {}", up, up.magnitude()).into());
                log_1(&format!("right: {:?}, norm: {}", right, right.magnitude()).into());
            }
            Message::SetFov(fov) => {
                self.camera.set_field_of_view(*fov);
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
    DrawGnomon(bool),
    DefaultCam,
    Debug,
    SetFov(f32),
}
