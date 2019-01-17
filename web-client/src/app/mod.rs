use camera::{ButtonState, Camera, MouseButton};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;

pub type AppWrapper = Rc<RefCell<App>>;

pub struct App {
    equation: String,
    pub clock: f32, 
    slider_pos: f32,
    checkbox: bool,
    camera: Camera,
}

impl App {
    pub fn new_wrapper() -> AppWrapper {
        Rc::new(RefCell::new(App {
            equation: format!("x + y"),
            clock: 0.0,
            slider_pos: 0.0,
            checkbox: false,
            camera: Camera::new(),
        }))
    }

    pub fn handle_message(&mut self, message: &Message) {
        match message {
            Message::MouseDown(x, y) => {
                self.camera.handle_mouse_input(MouseButton::Left, ButtonState::Pressed);
                self.camera.handle_mouse_move(*x as f32, *y as f32);
                log_1(&format!("clock: {}", self.clock).into());
            },
            Message::MouseUp => {
                self.camera.handle_mouse_input(MouseButton::Left, ButtonState::Released);
            }
            Message::MouseMove(x, y) => {
                self.camera.handle_mouse_move(*x as f32, *y as f32);
            },
            Message::Zoom(delta) => {
                self.camera.handle_scroll(*delta);
            },
            Message::EnterEquation(equation) => {
                self.equation.clear();
                self.equation += equation;
                log_1(&format!("Start parse").into());

                let input: Vec<char> = equation.chars().collect();
                
                let a = implicit_mesh::parser::parse_expression(&input, 0);

                log_1(&format!("Function Parse: {:?}", a).into());
            }
            Message::AdvanceClock(time_delta) => {
                self.clock += time_delta;

            }
            Message::SetSlider(position) => {
                self.slider_pos = *position;
            },
            Message::SetCheckbox(flag) => {
                self.checkbox = *flag
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
    AdvanceClock(f32),
    SetSlider(f32),
    SetCheckbox(bool),
}
