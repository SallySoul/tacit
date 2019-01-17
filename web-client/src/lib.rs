use console_error_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;
use web_sys::console::log_1;

use web_sys::window;

mod canvas;
mod controls;
mod app;
use crate::app::{App, AppWrapper, Message};

pub static APP_DIV_ID: &'static str = "tacit-app";

#[wasm_bindgen]
pub struct WebClient {
    app: AppWrapper,
    gl_context:  Rc<WebGlRenderingContext>
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();
        
        let app = App::new_wrapper();
        
        let gl_context = Rc::new(canvas::create_webgl_context(Rc::clone(&app)).unwrap());

        controls::append_controls(Rc::clone(&app)).expect("append_controls");

        WebClient { app, gl_context}
    }

    pub fn start(&self) {
        log_1(&"Start_1".into());
    }

    pub fn update(&self, time_delta: f32) {
        self.app.borrow_mut().handle_message(&Message::AdvanceClock(time_delta));
    }

    pub fn render(&self) {
        let t = self.app.borrow().clock;

        let x = t % 5000.0 / 5000.0;
        self.gl_context.clear_color(x, x, x, 1.);
        self.gl_context.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let above = 1000000.0;
        // Position is positive instead of negative for.. mathematical reasons..
        let clip_plane = [0., 1., 0., above];

    }
}


