//! Runner genérico para ejecutar aplicaciones de canvas

use std::cell::RefCell;
use std::rc::Rc;

use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlButtonElement, HtmlCanvasElement};

use crate::app::CanvasApp;
use crate::canvas::{get_canvas, get_context, get_document};

/// Estado interno del runner
struct RunnerState<A: CanvasApp> {
    app: A,
    canvas: HtmlCanvasElement,
    running: bool,
    minimized: bool,
    launched: bool,
    canvas_width: u32,
    canvas_height: u32,
}

/// Ejecuta una aplicación de canvas con los controles estándar del portfolio.
///
/// # Argumentos
/// * `id` - Sufijo de los IDs de elementos HTML (ej: "1" para "myCanvas1", "PlayPause1", etc.)
/// * `app` - Instancia de la aplicación que implementa `CanvasApp`
///
/// # Elementos HTML esperados
/// - `myCanvas{id}` - Canvas donde se dibuja
/// - `PlayPause{id}` - Botón play/pause
/// - `Reiniciar{id}` - Botón reiniciar
/// - `Minimizar{id}` - Botón minimizar
pub fn run_app<A: CanvasApp + 'static>(id: &str, app: A) {
    let document = get_document();

    let canvas = get_canvas(&format!("myCanvas{}", id));
    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    let state = Rc::new(RefCell::new(RunnerState {
        app,
        canvas: canvas.clone(),
        running: false,
        minimized: true,
        launched: false,
        canvas_width,
        canvas_height,
    }));

    // Obtener botones
    let play_button = document
        .get_element_by_id(&format!("PlayPause{}", id))
        .expect("Botón PlayPause no encontrado");
    let replay_button = document
        .get_element_by_id(&format!("Reiniciar{}", id))
        .expect("Botón Reiniciar no encontrado");
    let minimize_button = document
        .get_element_by_id(&format!("Minimizar{}", id))
        .expect("Botón Minimizar no encontrado");

    // Configurar observer de intersección para auto-pausar
    setup_intersection_observer(&canvas, Rc::clone(&state));

    // Configurar input de teclado
    setup_keyboard_input(&canvas, Rc::clone(&state));

    // Configurar botones
    setup_play_button(&play_button, Rc::clone(&state));
    setup_replay_button(&replay_button, Rc::clone(&state));
    setup_minimize_button(&minimize_button, Rc::clone(&state));
}

fn setup_intersection_observer<A: CanvasApp + 'static>(
    canvas: &HtmlCanvasElement,
    state: Rc<RefCell<RunnerState<A>>>,
) {
    let state_clone = Rc::clone(&state);

    let closure = Closure::wrap(Box::new(move |entries: web_sys::js_sys::Array| {
        for entry in entries.iter() {
            let intersection_entry = entry
                .dyn_into::<web_sys::IntersectionObserverEntry>()
                .unwrap();
            if !intersection_entry.is_intersecting() {
                let mut s = state_clone.borrow_mut();
                if !s.minimized {
                    s.minimized = true;
                    if s.running {
                        s.running = false;
                    }
                    s.canvas.set_height(0);
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    let options = web_sys::IntersectionObserverInit::new();
    let observer =
        web_sys::IntersectionObserver::new_with_options(closure.as_ref().unchecked_ref(), &options)
            .expect("Error creando IntersectionObserver");
    observer.observe(canvas);
    closure.forget();
}

fn setup_keyboard_input<A: CanvasApp + 'static>(
    canvas: &HtmlCanvasElement,
    state: Rc<RefCell<RunnerState<A>>>,
) {
    let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut s = state.borrow_mut();
        s.app.on_key(&event.key());
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .expect("Error añadiendo listener de teclado");
    closure.forget();
}

fn setup_play_button<A: CanvasApp + 'static>(
    button: &Element,
    state: Rc<RefCell<RunnerState<A>>>,
) {
    let button_clone = button.clone();
    let state_for_closure = Rc::clone(&state);

    let closure = Closure::wrap(Box::new(move || {
        let should_start;
        {
            let mut s = state_for_closure.borrow_mut();
            s.running = !s.running;

            // Actualizar icono del botón
            if let Some(icon) = button_clone.first_child() {
                let icon_text = if s.running { "pause" } else { "play_arrow" };
                icon.set_text_content(Some(icon_text));
            }

            should_start = !s.launched && s.running;

            if should_start {
                s.launched = true;
                // Expandir canvas si está minimizado
                if s.minimized {
                    s.minimized = false;
                    s.canvas.set_height(s.canvas_height);
                }
            }
        }

        if should_start {
            start_game_loop(Rc::clone(&state_for_closure));
        }
    }) as Box<dyn Fn()>);

    button
        .clone()
        .dyn_into::<HtmlButtonElement>()
        .expect("No es un botón")
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Error añadiendo listener");
    closure.forget();
}

fn setup_replay_button<A: CanvasApp + 'static>(
    button: &Element,
    state: Rc<RefCell<RunnerState<A>>>,
) {
    let closure = Closure::wrap(Box::new(move || {
        let (canvas_width, canvas_height);
        {
            let mut s = state.borrow_mut();

            // Expandir canvas si está minimizado
            if s.minimized {
                s.minimized = false;
                s.canvas.set_height(s.canvas_height);
            }

            canvas_width = s.canvas_width;
            canvas_height = s.canvas_height;

            // Reinicializar la aplicación
            s.app.init(canvas_width, canvas_height);
            s.running = true;
            s.launched = true;
        }

        start_game_loop(Rc::clone(&state));
    }) as Box<dyn Fn()>);

    button
        .clone()
        .dyn_into::<HtmlButtonElement>()
        .expect("No es un botón")
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Error añadiendo listener");
    closure.forget();
}

fn setup_minimize_button<A: CanvasApp + 'static>(
    button: &Element,
    state: Rc<RefCell<RunnerState<A>>>,
) {
    let closure = Closure::wrap(Box::new(move || {
        let mut s = state.borrow_mut();
        s.minimized = !s.minimized;

        if s.minimized {
            s.running = false;
            s.canvas.set_height(0);
        } else {
            s.canvas.set_height(s.canvas_height);
            // Redibujar al expandir
            let ctx = get_context(&s.canvas);
            s.app.draw(&ctx);
        }
    }) as Box<dyn Fn()>);

    button
        .clone()
        .dyn_into::<HtmlButtonElement>()
        .expect("No es un botón")
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Error añadiendo listener");
    closure.forget();
}

fn start_game_loop<A: CanvasApp + 'static>(state: Rc<RefCell<RunnerState<A>>>) {
    spawn_local(async move {
        // Inicializar la aplicación
        {
            let mut s = state.borrow_mut();
            let (w, h) = (s.canvas_width, s.canvas_height);
            s.app.init(w, h);
        }

        loop {
            let delay;
            let should_continue;

            {
                let s = state.borrow();
                delay = s.app.frame_delay();
            }

            TimeoutFuture::new(delay).await;

            {
                let mut s = state.borrow_mut();

                if !s.running || s.minimized {
                    continue;
                }

                // Update
                should_continue = s.app.update();

                // Draw
                let ctx = get_context(&s.canvas);
                s.app.draw(&ctx);
            }

            if !should_continue {
                let mut s = state.borrow_mut();
                s.running = false;
                s.launched = false;
                break;
            }
        }
    });
}
