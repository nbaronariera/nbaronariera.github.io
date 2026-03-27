use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn get_document() -> Document {
    window().unwrap().document().unwrap()
}

fn get_canvas(id: &str) -> HtmlCanvasElement {
    get_document()
        .get_element_by_id(&format!("myCanvas{}", id))
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap()
}

/// Función principal que se llama desde JavaScript
/// El id corresponde al canvas que se usará
#[wasm_bindgen]
pub fn start_system(id: &str) -> Result<(), JsValue> {
    let canvas = get_canvas(id);
    let ctx = get_context(&canvas);

    // Ajustar altura del canvas
    canvas.set_height(400);

    // Ejemplo: dibujar un rectángulo
    ctx.set_fill_style_str("blue");
    ctx.fill_rect(50.0, 50.0, 100.0, 100.0);

    // Ejemplo: escribir texto
    ctx.set_fill_style_str("black");
    ctx.set_font("20px Arial");
    ctx.fill_text("¡Hola desde WASM!", 50.0, 200.0)?;

    console_log!("Módulo WASM iniciado correctamente");

    Ok(())
}
