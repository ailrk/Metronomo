use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, Element, HtmlInputElement};


struct State {
    is_playing: bool,
    interval_id: Option<i32>,
    ctx: AudioContext,
    slider: HtmlInputElement,
    btn: Element,
}

#[derive(Clone)]
struct Metronomo(std::rc::Rc<RefCell<State>>);


impl Metronomo {
    fn start() -> Result<Self, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have document on window");
        let body = document.body().expect("document should have a body");

        let title = document.create_element("h1")?;
        title.set_inner_html("Metronomo");
        body.append_child(&title)?;

        let slider = document
            .create_element("input")?
            .dyn_into::<HtmlInputElement>()?;
        slider.set_type("range");
        slider.set_min("20");
        slider.set_max("218");
        slider.set_value("120");
        body.append_child(&slider)?;

        let btn = document
            .create_element("button")?;
        btn.set_inner_html("START");
        body.append_child(&btn)?;

        let state = Metronomo(
            Rc::new(RefCell::new(State {
                is_playing: false,
                interval_id: None,
                ctx: AudioContext::new()?,
                slider: slider,
                btn: btn
            })));

        // btn callback
        let btn_state = state.clone();
        let btn_closure = Closure::wrap(Box::new(move || {
            let mut data = btn_state.lock();
            if data.is_playing {
                if let Some(id) = data.interval_id {
                    web_sys::window().unwrap().clear_interval_with_handle(id);
                }
                data.btn.set_inner_html("START");
                data.is_playing = false;
            } else {
                data.btn.set_inner_html("STOP");
                data.is_playing = true;
                drop(data);
                btn_state.start_loop();
            }
        }) as Box<dyn FnMut()>);

        state.lock()
            .btn
            .add_event_listener_with_callback("click", btn_closure.as_ref().unchecked_ref())?;
        btn_closure.forget();

        // slider callback
        let slider_state = state.clone();
        let slider_closure = Closure::wrap(Box::new(move |_e| {
            if slider_state.lock().is_playing {
                slider_state.start_loop();
            }
        }) as Box<dyn FnMut(web_sys::EventTarget)>);

        state.lock()
            .slider
            .add_event_listener_with_callback("input", slider_closure.as_ref().unchecked_ref())?;
        slider_closure.forget();

        Ok (state)
    }

    fn lock(&self) -> RefMut<'_, State> {
        self.0.borrow_mut()
    }

    fn start_loop(&self) {
        let mut data = self.lock();

        // clear the existing interval
        if let Some(id) = data.interval_id {
            web_sys::window().unwrap().clear_interval_with_handle(id);
        }

        // create new ticker
        let bpm: f64 = data.slider.value().parse().unwrap_or(120.0);
        let ms = (60.0 / bpm * 1000.0) as i32;
        let audio_ctx = data.ctx.clone();
        let tick = Closure::wrap(Box::new(move || {
            let osc = audio_ctx.create_oscillator().unwrap();
            osc.connect_with_audio_node(&audio_ctx.destination()).unwrap();
            osc.start().unwrap();
            osc.stop_with_when(audio_ctx.current_time() + 0.05).unwrap();
        }) as Box<dyn FnMut()>);

        // install ticker
        let id = web_sys::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                tick.as_ref().unchecked_ref(), ms).unwrap();
        data.interval_id = Some(id) ;
        tick.forget();
    }
}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    Metronomo::start()?;
    Ok (())
}
