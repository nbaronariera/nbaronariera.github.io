//! Helpers para trabajar con canvas HTML5

use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d};

/// Obtiene el Document del navegador
pub fn get_document() -> Document {
    window()
        .expect("No hay window disponible")
        .document()
        .expect("No hay document disponible")
}

/// Obtiene un canvas por su ID
pub fn get_canvas(id: &str) -> HtmlCanvasElement {
    get_document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Canvas con id '{}' no encontrado", id))
        .dyn_into::<HtmlCanvasElement>()
        .expect("El elemento no es un HtmlCanvasElement")
}

/// Obtiene el contexto 2D de un canvas
pub fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Error obteniendo contexto")
        .expect("Contexto 2d no disponible")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("No se pudo convertir a CanvasRenderingContext2d")
}

/// Limpia el canvas con un color de fondo
pub fn clear_canvas(ctx: &CanvasRenderingContext2d, width: u32, height: u32, color: &str) {
    ctx.set_fill_style_str(color);
    ctx.fill_rect(0.0, 0.0, width as f64, height as f64);
}

/// Dibuja un rectángulo relleno
pub fn fill_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64, color: &str) {
    ctx.set_fill_style_str(color);
    ctx.fill_rect(x, y, width, height);
}

/// Dibuja texto en el canvas
pub fn draw_text(ctx: &CanvasRenderingContext2d, text: &str, x: f64, y: f64, font: &str, color: &str) {
    ctx.set_font(font);
    ctx.set_fill_style_str(color);
    let _ = ctx.fill_text(text, x, y);
}
