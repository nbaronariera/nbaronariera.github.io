mod app;
mod snake;

use wasm_bindgen::prelude::*;
use wasm_canvas_lib::run_app;

use crate::app::SnakeApp;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn start_system(id: &str) {
    run_app(id, SnakeApp::new());
}
