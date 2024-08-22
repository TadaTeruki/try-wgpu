use log::error;
use state::State;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod camera;
mod earth;
mod fetch;
mod key;
mod star;
mod state;
mod sun;

#[wasm_bindgen(start)]
fn start() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
pub async fn create_state(
    canvas: web_sys::HtmlCanvasElement,
    use_gl_instead: bool,
) -> Option<State> {
    match State::new(canvas, use_gl_instead).await {
        Ok(state) => Some(state),
        Err(e) => {
            error!("Failed to create state: {:?}", e);
            None
        }
    }
}
