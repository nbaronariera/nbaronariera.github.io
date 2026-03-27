use crate::snake::{launch, draw};
use crate::state::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::js_sys;
use web_sys::{window, HtmlButtonElement, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn start_system(id: &str) -> Result<(), JsValue> {
    set_start(Some(launch));

    let document = window().unwrap().document().expect("Expected document");

    let play_button = document
        .get_element_by_id(&("PlayPause".to_owned() + id))
        .expect(&format!("Button Pause with id {id} not found"));
    let replay_button = document
        .get_element_by_id(&("Reiniciar".to_owned() + id))
        .expect(&format!("Button Reset id {id} not found"));
    let minimize_button = document
        .get_element_by_id(&("Minimizar".to_owned() + id))
        .expect(&format!("Button Minimize id {id} not found"));

    set_canvas(Some(
        document
            .get_element_by_id(&("myCanvas".to_owned() + id)).expect("Expected element")
            .dyn_into::<HtmlCanvasElement>().expect("Expected canvas")
    ));

    set_playpause(Some(play_button.clone()));

    let observer_closure = Closure::wrap(Box::new(move |entries: js_sys::Array| {
        for entry in entries.iter() {
            let intersection_entry = entry
                .dyn_into::<web_sys::IntersectionObserverEntry>()
                .unwrap();
            if !intersection_entry.is_intersecting() {
                set_minimize(false);
                minimize_canvas();
            }
        }
    }) as Box<dyn FnMut(_)>);

    let options = web_sys::IntersectionObserverInit::new();
    let observer = web_sys::IntersectionObserver::new_with_options(
        observer_closure.as_ref().unchecked_ref(),
        &options,
    )?;
    observer.observe(&get_canvas().unwrap());
    observer_closure.forget();

    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if let Some(f) = get_input() {
            let _ = f(event.key());
        }
    }) as Box<dyn FnMut(_)>);
    get_canvas()
        .unwrap()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();

    let _ = set_button(&play_button, playpause);
    let _ = set_button(&replay_button, restart);
    let _ = set_button(&minimize_button, minimize_canvas);

    Ok(())
}

pub fn playpause() {
    set_running(!get_running());

    if !get_launched() {
        restart();
    }

    if get_running() && get_minimize() {
        minimize_canvas();
    }

    if let Some(button) = get_playpause() {
        if let Some(icon) = button.first_child() {
            let icon_text = if get_running() { "pause" } else { "play_arrow" };

            icon.set_text_content(Some(icon_text));
        }
    }
}

pub fn restart() {
    if get_minimize() {
        minimize_canvas();
    }

    if !get_launched() {
        set_launched(true);
    }

    if let Some(f) = get_start() {
        let _ = f();
    }
}

pub fn minimize_canvas() {
    set_minimize(!get_minimize());

    if get_running() && get_minimize() {
        playpause();
    }

    if let Some(canvas) = get_canvas() {
        let aspect_ratio = if get_minimize() { 0 } else { 720 };
        canvas.set_height(aspect_ratio);
        draw();
    } else {
        web_sys::console::error_1(&JsValue::from_str("Canvas element not found"));
    }
}

fn set_button(button: &web_sys::Element, func: fn()) -> Result<(), JsValue> {
    let button = button.clone().dyn_into::<HtmlButtonElement>()?;
    let closure = Closure::wrap(Box::new(move || func()) as Box<dyn Fn()>);

    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}
