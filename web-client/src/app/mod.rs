use std::cell::RefCell;
use std::rc::Rc;
use web_sys::console::log_1;

pub type AppWrapper = Rc<RefCell<App>>;

pub struct App {
    mouse_pos: (i32, i32),
    mouse_pressed: bool,
    zoom_counter: f32,
    equation: String,
    clock: f32, 
    slider_pos: f32,
    checkbox: bool,
}

impl App {
    pub fn new_wrapper() -> AppWrapper {
        Rc::new(RefCell::new(App {
            mouse_pos: (0, 0),
            mouse_pressed: false,
            zoom_counter: 0.0,
            equation: format!("x + y"),
            clock: 0.0,
            slider_pos: 0.0,
            checkbox: false,
        }))
    }

    pub fn handle_message(&mut self, message: &Message) {
        match message {
            Message::MouseDown(x, y) => {
                self.mouse_pressed = true;
                self.mouse_pos = (*x, *y);
                log_1(&format!("Mouse Down: {}, {}", x, y).into());
            },
            Message::MouseUp => {
                self.mouse_pressed = false;
            }
            Message::MouseMove(x, y) => {
                self.mouse_pressed = true;
                self.mouse_pos = (*x, *y);
            },
            Message::Zoom(delta) => {
                self.zoom_counter += delta;
            },
            Message::EnterEquation(equation) => {
                self.equation.clear();
                self.equation += equation;
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
