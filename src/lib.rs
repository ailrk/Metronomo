use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, HtmlInputElement};


struct StateData {
    is_playing: bool,
    interval_id: Option<i32>,
    ctx: AudioContext,
}

#[derive(Clone)]
struct State(std::rc::Rc<RefCell<StateData>>);


impl State {
    fn new() -> Self{
        State(
            Rc::new(RefCell::new(StateData {
                is_playing: false,
                interval_id: None,
                ctx: AudioContext::new().unwrap() })))
    }

    fn lock(&self) -> RefMut<'_, StateData> {
        self.0.borrow_mut()
    }
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have document on window");
    let body = document.body().expect("document should have a body");

    let title = document.create_element("h1")?;
    title.set_inner_html("Metronomo");
    body.append_child(&title)?;

    let slider = document.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    slider.set_type("range");
    slider.set_min("20");
    slider.set_max("218");
    slider.set_value("120");
    body.append_child(&slider)?;

    let btn = document.create_element("button")?;
    btn.set_inner_html("START");
    body.append_child(&btn)?;

    let state = State::new();


    // btn callback
    let slider_handle = slider.clone();
    let btn_handle = btn.clone();
    let window_handle = window.clone();
    let btn_state = state.clone();
    let btn_closure = Closure::wrap(Box::new(move || {
        let mut data = btn_state.lock();
        if data.is_playing {
            if let Some(id) = data.interval_id {
                web_sys::window().unwrap().clear_interval_with_handle(id);
            }
            btn_handle.set_inner_html("START");
            data.is_playing = false;
        } else {
            let bpm: f64 = slider_handle.value().parse().unwrap_or(120.0);
            let ms = (60.0 / bpm * 1000.0) as i32;
            let audio_ctx = data.ctx.clone();
            let tick = Closure::wrap(Box::new(move || {
                let osc = audio_ctx.create_oscillator().unwrap();
                osc.connect_with_audio_node(&audio_ctx.destination()).unwrap();
                osc.start().unwrap();
                osc.stop_with_when(audio_ctx.current_time() + 0.05).unwrap();
            }) as Box<dyn FnMut()>);

            data.interval_id = {
                let id = window_handle
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        tick.as_ref().unchecked_ref(), ms).unwrap();
                Some(id)
            };
            tick.forget();
            btn_handle.set_inner_html("STOP");
            data.is_playing = true;
        }
    }) as Box<dyn FnMut()>);

    btn.add_event_listener_with_callback("click", btn_closure.as_ref().unchecked_ref())?;
    btn_closure.forget();

    // slider callback
    let window_handle = window.clone();
    let slider_handle = slider.clone();
    let slider_state = state.clone();
    let slider_closure = Closure::wrap(Box::new(move |_e| {
        let mut data = slider_state.lock();
        if data.is_playing {
            if let Some(id) = data.interval_id {
                web_sys::window().unwrap().clear_interval_with_handle(id);
            }
            let bpm: f64 = slider_handle.value().parse().unwrap_or(120.0);
            let ms = (60.0 / bpm * 1000.0) as i32;
            let audio_ctx = data.ctx.clone();
            let tick = Closure::wrap(Box::new(move || {
                let osc = audio_ctx.create_oscillator().unwrap();
                osc.connect_with_audio_node(&audio_ctx.destination()).unwrap();
                osc.start().unwrap();
                osc.stop_with_when(audio_ctx.current_time() + 0.05).unwrap();
            }) as Box<dyn FnMut()>);

            data.interval_id = {
                let id = window_handle
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        tick.as_ref().unchecked_ref(), ms).unwrap();
                Some(id)
            };
            tick.forget();

        }
    }) as Box<dyn FnMut(web_sys::EventTarget)>);

    slider.add_event_listener_with_callback("input", slider_closure.as_ref().unchecked_ref())?;
    slider_closure.forget();

    Ok (())
}
