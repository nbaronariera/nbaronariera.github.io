//! # wasm-canvas-lib
//!
//! Librería compartida para crear aplicaciones WASM que usan canvas HTML5.
//!
//! ## Uso básico
//!
//! ```rust
//! use wasm_canvas_lib::{CanvasApp, run_app};
//! use web_sys::CanvasRenderingContext2d;
//!
//! struct MiApp {
//!     frame: u32,
//! }
//!
//! impl MiApp {
//!     fn new() -> Self {
//!         Self { frame: 0 }
//!     }
//! }
//!
//! impl CanvasApp for MiApp {
//!     fn init(&mut self, _width: u32, _height: u32) {
//!         self.frame = 0;
//!     }
//!
//!     fn update(&mut self) -> bool {
//!         self.frame += 1;
//!         true
//!     }
//!
//!     fn draw(&self, ctx: &CanvasRenderingContext2d) {
//!         ctx.set_fill_style_str("blue");
//!         ctx.fill_rect(0.0, 0.0, 100.0, 100.0);
//!     }
//! }
//!
//! #[wasm_bindgen]
//! pub fn start_system(id: &str) {
//!     run_app(id, MiApp::new());
//! }
//! ```

mod app;
mod canvas;
mod runner;
mod state;

// Re-exports públicos
pub use app::CanvasApp;
pub use canvas::{clear_canvas, draw_text, fill_rect, get_canvas, get_context, get_document};
pub use runner::run_app;

// Re-export de la macro
pub use paste;

// Re-exports de web-sys para conveniencia
pub use web_sys::CanvasRenderingContext2d;
