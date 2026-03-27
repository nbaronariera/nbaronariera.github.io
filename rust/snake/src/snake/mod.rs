mod structures;
mod local_state;

use crate::snake::structures::*;
use crate::snake::local_state::*;
use crate::{get_canvas, set_running, state};
use gloo_timers::future::TimeoutFuture;
use rand::prelude::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d};

static SIZE: u32 = 40;

#[wasm_bindgen()]
pub fn launch() -> Result<(), JsValue> {
    set_exit(true);
    state::set_input(Some(controls));

    let canvas = get_canvas().unwrap();
    set_canvas_width(canvas.width());
    set_canvas_height(canvas.height());

    set_max_x(get_canvas_width() / SIZE);
    set_max_y(get_canvas_height() / SIZE);

    assert_eq!(get_canvas_width() % SIZE, 0, "El ancho no es divisible entre el escalado");
    assert_eq!(get_canvas_height() % SIZE, 0, "El alto no es divisible entre el escalado");

    set_exit(false);
    set_running(true);
    set_puntos(0);

    spawn_local(async {
        let mut rng = rand::thread_rng();

        set_snake(Some(Snake::new((get_max_x() / 2, get_max_y() / 2))));
        set_manzana((
            rng.gen_range(1..(get_max_x() - 1)),
            rng.gen_range(1..(get_max_y() - 1)),
        ));

        loop {
            TimeoutFuture::new(100).await;
            if state::get_running() && !state::get_minimize() {
                let mut snake = get_snake().unwrap();

                if get_exit() || snake.check_internal_collision() {
                    draw_exit();
                    break;
                }

                if snake.get_position() == get_manzana() {
                    set_puntos(get_puntos() + 1);

                    set_manzana((
                        rng.gen_range(1..(get_max_x() - 1)),
                        rng.gen_range(1..(get_max_y() - 1)),
                    ));
                    snake.add_part();
                }
                snake.move_by();
                set_snake(Some(snake));

                draw();
            }
        }
    });

    Ok(())
}

fn draw_exit(){
    let context = get_canvas().unwrap()
        .get_context("2d").unwrap()
        .expect("Expected context")
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
        .expect("Expected Rendering Context");
    context.set_font("bold 20px serif");
    let _ = context.fill_text(
        &format!("Fin de la partida, puntuación: {}", get_puntos()),
        (get_canvas_width() / 3) as f64,
        (get_canvas_height() / 2) as f64,
    );

}
fn controls(key: String) {
    let mut snake = get_snake().unwrap();
    match key.as_str() {
        "d" if snake.get_direction() != Direction::LEFT => {
            snake.change_direction(Direction::RIGHT);
        }
        "a" if snake.get_direction() != Direction::RIGHT => {
            snake.change_direction(Direction::LEFT);
        }
        "s" if snake.get_direction() != Direction::UP => {
            snake.change_direction(Direction::DOWN);
        }
        "w" if snake.get_direction() != Direction::DOWN => {
            snake.change_direction(Direction::UP);
        }
        "Escape" => {
            set_exit(true);
        }
        _ => {
            web_sys::console::log_1(&JsValue::from_str(&format!("No capturada {key}")));
        }
    }
    set_snake(Some(snake));
}

pub fn draw() {
    let snake = get_snake().unwrap();
    let canvas = get_canvas().unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .expect("Expected context")
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()
        .expect("Expected Rendering Context");

    //Limpiar frame
    context.set_fill_style_str("gray");
    context.fill_rect(
        0.0,
        0.0,
        get_canvas_width() as f64,
        get_canvas_height() as f64,
    );

    //Dibujar manzana
    context.set_fill_style_str("red");
    context.fill_rect(
        (get_manzana().0 * SIZE) as f64,
        (get_manzana().1 * SIZE) as f64,
        SIZE as f64,
        SIZE as f64,
    );


    //Dibujar cuerpos
    context.set_fill_style_str("green");
    snake.get_positions().iter().fold(0,|_, b| {
        context.fill_rect(
            (b.0 * SIZE) as f64,
            (b.1 * SIZE) as f64,
            SIZE as f64,
            SIZE as f64,
        );
        0}
    );

    //Dibujar cabeza
    context.set_fill_style_str("darkgreen");
    context.fill_rect(
        (snake.get_position().0 * SIZE) as f64,
        (snake.get_position().1 * SIZE) as f64,
        SIZE as f64,
        SIZE as f64,
    );

    //Dibujar bordes
    context.set_fill_style_str("black");

    context.fill_rect(0.0, 0.0, get_canvas_width() as f64, SIZE as f64);
    context.fill_rect(
        0.0,
        (get_canvas_height() - SIZE) as f64,
        get_canvas_width() as f64,
        SIZE as f64,
    );
    context.fill_rect(0.0, 0.0, SIZE as f64, get_canvas_height() as f64);
    context.fill_rect(
        (get_canvas_width() - SIZE) as f64,
        0.0,
        SIZE as f64,
        get_canvas_height() as f64,
    );
}
