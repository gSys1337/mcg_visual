// pub mod example;
pub mod game;
mod utils;

// #[cfg(target_arch = "wasm32")]
use eframe::WebRunner;
// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Array;
use web_sys::js_sys::Promise;

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn handleDirectorySelection() -> Array;
    pub fn openDirectoryPicker() -> Promise;
}

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(game::App::new(cc)))),
            )
            .await
            .expect("Failed to start eframe");
    });
    Ok(())
}

/* TODO implement right-click with popup when this
    https://github.com/emilk/egui/blob/master/crates/egui/src/containers/popup.rs
    gets into a proper egui release
*/
