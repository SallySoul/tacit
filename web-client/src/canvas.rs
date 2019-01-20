use crate::app::{AppWrapper, Message};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use crate::APP_DIV_ID;

pub static CANVAS_WIDTH: i32 = 512;
pub static CANVAS_HEIGHT: i32 = 512;

pub fn create_webgl_context(app: AppWrapper) -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = init_canvas(app)?;

    let gl: WebGl2RenderingContext = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

    Ok(gl)
}

fn init_canvas(app: AppWrapper) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(CANVAS_WIDTH as u32);
    canvas.set_height(CANVAS_HEIGHT as u32);

    attach_mouse_down_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_up_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_move_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_wheel_handler(&canvas, Rc::clone(&app))?;

    attach_touch_start_handler(&canvas, Rc::clone(&app))?;
    attach_touch_move_handler(&canvas, Rc::clone(&app))?;
    attach_touch_end_handler(&canvas, Rc::clone(&app))?;

    let app_div: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into()?,
        None => {
            let app_div = document.create_element("div")?;
            app_div.set_id(APP_DIV_ID);
            app_div.dyn_into()?
        }
    };

    // Layout of div
    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        app.borrow_mut().handle_message(&Message::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    //  As long as the app is open we'd never want to deallocate this, so lets not
    //  worry about the paperwork
    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::MouseEvent| {
        app.borrow_mut().handle_message(&Message::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        app.borrow_mut().handle_message(&Message::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();

        let zoom_amount = event.delta_y();

        app.borrow_mut()
            .handle_message(&Message::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_start_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        app.borrow_mut().handle_message(&Message::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_move_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        event.prevent_default();
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        app.borrow_mut().handle_message(&Message::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_end_handler(canvas: &HtmlCanvasElement, app: AppWrapper) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::TouchEvent| {
        app.borrow_mut().handle_message(&Message::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback("touchend", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
