//! Trait principal para aplicaciones de canvas (demos, visualizaciones, juegos, etc.)

use web_sys::CanvasRenderingContext2d;

/// Trait que define una aplicación de canvas.
///
/// Implementa este trait para crear demos, visualizaciones, simulaciones o juegos
/// que se ejecutan en un canvas HTML5.
///
/// # Ejemplo
/// ```rust
/// use wasm_canvas_lib::{CanvasApp, run_app};
///
/// struct MiApp {
///     frame: u32,
/// }
///
/// impl CanvasApp for MiApp {
///     fn init(&mut self, _width: u32, _height: u32) {
///         self.frame = 0;
///     }
///
///     fn update(&mut self) -> bool {
///         self.frame += 1;
///         true // continuar
///     }
///
///     fn draw(&self, ctx: &CanvasRenderingContext2d) {
///         ctx.set_fill_style_str("blue");
///         ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
///     }
/// }
/// ```
pub trait CanvasApp {
    /// Inicializa la aplicación.
    /// Se llama una vez al expandir el canvas.
    fn init(&mut self, canvas_width: u32, canvas_height: u32);

    /// Actualiza el estado de la aplicación.
    /// Se llama cada frame antes de dibujar.
    /// Devuelve `false` para terminar el loop, `true` para continuar.
    fn update(&mut self) -> bool;

    /// Dibuja el estado actual en el canvas.
    fn draw(&self, ctx: &CanvasRenderingContext2d);

    /// Maneja input de teclado.
    /// Implementar solo si la aplicación necesita input.
    fn on_key(&mut self, _key: &str) {}

    /// Delay entre frames en milisegundos.
    /// Por defecto: 100ms (10 FPS).
    fn frame_delay(&self) -> u32 {
        100
    }

    /// Color de fondo para limpiar el canvas cada frame.
    /// Por defecto: "gray".
    fn background_color(&self) -> &str {
        "gray"
    }
}
