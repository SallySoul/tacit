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

    pub fn handle_message(&mut self, message: &Message) {
        match message {
            Message::MouseDown(x, y) => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Pressed);
                self.camera.handle_mouse_move(*x as f32, *y as f32);

                log_1(&format!("cam: {:?}", self.camera.get_clipspace_transform()).into());
            }
            Message::MouseUp => {
                self.camera
                    .handle_mouse_input(MouseButton::Left, ButtonState::Released);
            }
            Message::MouseMove(x, y) => {
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            }
            Message::Zoom(delta) => {
                log_1(&format!("Zoom!").into());
                self.camera.handle_scroll(*delta);
            }
            Message::EnterEquation(equation) => {
                self.equation.clear();
                self.equation += equation;
                log_1(&format!("Start parse").into());

                let input: Vec<char> = equation.chars().collect();

                let a = implicit_mesh::parser::parse_expression(&input, 0).expect("not parseable");
                let size_interval = Interval::new(-40.0 / 2.0, 40.0 / 2.0);
                let bounding_box = BoundingBox {
                    x: size_interval.clone(),
                    y: size_interval.clone(),
                    z: size_interval.clone(),
                };

                self.mtree = Some(MeshTree::new(a, bounding_box));

                log_1(&"made mesh tree".into());
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
                        mtree.generate_vertex_map();
                        mtree.generate_triangle_set();
                        mtree.add_to_plot(true, true, true, true, &mut plot);
                    }
                    None => {
                        log_1(&"App: no mtree to next level".into());
                        return;
                    }
                };

                match &mut self.renderer {
                    Some(renderer) => {
                        renderer
                            .borrow_mut()
                            .set_plot(&plot)
                            .expect("Unable to set plot");
                    }
                    None => (),
                };
            }
            Message::Relax => {
                log_1(&"App: Relax".into());

                match &mut self.mtree {
                    Some(mtree) => {
                        mtree.relax_vertices();
                    }
                    None => {
                        log_1(&"App: no mtree to relax".into());
                        return;
                    }
                };
                /*
                match self.renderer {
                    Some(renderer) => {
                        let mut plot = Plot::new();
                        self.mtree.unwrap().add_to_plot(false, true, false, true, &mut plot);

                        self.renderer.unwrap().borrow_mut().set_plot(&plot);
                    }
                    None => ()
                };
                */
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
}
