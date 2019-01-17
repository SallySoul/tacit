use crate::APP_DIV_ID;
use crate::app::{AppWrapper, Message};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::console::log_1;

pub fn append_controls(app: AppWrapper) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    let controls = document.create_element("div")?;
    container.append_child(&controls)?;
    let controls: HtmlElement = controls.dyn_into()?;
    controls.style().set_property("padding-left", "5px")?;
    let controls: Element = controls.dyn_into()?;

    // Reflectivity
    {
        let app = Rc::clone(&app);
        let reflectivity_control = create_reflectivity_control(app)?;
        controls.append_child(&reflectivity_control)?;
    }

    // Use Refraction
    {
        let app = Rc::clone(&app);
        let use_refraction_control = create_use_refraction_checkbox(app)?;
        controls.append_child(&use_refraction_control)?;
    }

    {
        let app = Rc::clone(&app);
        let text_input = create_text_input(app)?;
        controls.append_child(&text_input)?;
    }

    Ok(())
}

fn create_reflectivity_control(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let reflectivity = input_elem.value().parse().unwrap();

        app
            .borrow_mut()
            .handle_message(&Message::SetSlider(reflectivity));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let reflectivity_control = Slider {
        min: 0.0,
        max: 1.0,
        step: 0.1,
        start: 0.5,
        label: "Slider",
        closure,
    }
    .create_element()?;

    Ok(reflectivity_control)
}

fn create_use_refraction_checkbox(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let use_refraction = input_elem.checked();

        app
            .borrow_mut()
            .handle_message(&Message::SetCheckbox(use_refraction));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let use_refraction_control = Checkbox {
        start_checked: true,
        label: "Use Refraction",
        closure,
    }
    .create_element()?;

    Ok(use_refraction_control)
}

fn create_text_input(app: AppWrapper) -> Result<HtmlElement, JsValue> {
    log_1(&"Started create_text_input".into());
/*
    let handler = move |event: web_sys::Event| {
        log_1(&format!("Event: {:?}", event).into());
        let window = window().unwrap();
        let document = window.document().unwrap();

        let text_box: HtmlInputElement = document.get_element_by_id("tacit_equation_entry_box").expect("Could not get text box").dyn_into().expect("Text box dyn into");
        let text: String = text_box.value();

        app
            .borrow_mut()
            .handle_message(&Message::EnterEquation(text));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
    log_1(&"1 create_text_input".into());
    let window = window().unwrap();
    let document = window.document().unwrap();

    let result = document.create_element("div")?;

    let group: HtmlElement = result.dyn_into()?;
    let button: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        button.set_type("submit");
        button.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

    let text_box: HtmlInputElement = document.create_element("text")?.dyn_into()?;
        text_box.set_id("tacit_equation_entry_box");
    
    group.append_child(&button)?;
    group.append_child(&text_box)?;
*/
    let handler = move |event: web_sys::Event| {
        log_1(&format!("Event: {:?}", event).into());
        let window = window().unwrap();
        let document = window.document().unwrap();

        let text_box: HtmlInputElement = document.get_element_by_id("tacit_equation_entry_box").expect("Could not get text box").dyn_into().expect("Text box dyn into");
        let text: String = text_box.value();

        log_1(&format!("Text: {}", text).into());
        app
            .borrow_mut()
            .handle_message(&Message::EnterEquation(text));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);


    let window = window().unwrap();
    let document = window.document().unwrap();
    let text_input: HtmlElement = document.create_element("div")?.dyn_into()?;
    log_1(&"Finished create_text_input".into());

    let text_box: HtmlInputElement = document.create_element("input")?.dyn_into()?;
    text_box.set_type("text");
    text_box.set_id("tacit_equation_entry_box");
    log_1(&"made_text_box".into());

    text_input.append_child(&text_box)?;
    log_1(&"appdned text b".into());

    let button: HtmlInputElement = document.create_element("input")?.dyn_into()?;
    button.set_type("button");
    button.set_value("Submit Equation");
    log_1(&"added button".into());

    text_input.append_child(&button)?;
    log_1(&"appended button".into());

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    log_1(&"appended closure".into());


    Ok(text_input)
}

struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Slider {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let slider: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        slider.set_type("range");
        slider.set_min(&format!("{}", self.min));
        slider.set_max(&format!("{}", self.max));
        slider.set_step(&format!("{}", self.step));
        slider.set_value(&format!("{}", self.start));

        let closure = self.closure;
        slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("div")?;
        label.set_inner_html(self.label);

        let container = document.create_element("div")?;
        container.append_child(&label)?;
        container.append_child(&slider)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;

        Ok(container)
    }
}

struct Checkbox {
    start_checked: bool,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Checkbox {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let checkbox: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        checkbox.set_type("checkbox");
        checkbox.set_checked(self.start_checked);

        let closure = self.closure;
        checkbox.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("label")?;
        label.set_inner_html(self.label);
        label.append_child(&checkbox)?;

        let container = document.create_element("div")?;
        container.append_child(&label)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;
        container.style().set_property("display", "flex")?;
        container.style().set_property("align-items", "center")?;
        container.style().set_property("cursor", "pointer")?;

        Ok(container)
    }
}
