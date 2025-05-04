pub mod example;
pub mod game;
mod utils;

#[allow(unused_imports)]
use eframe::AppCreator;
#[cfg(target_arch = "wasm32")]
use eframe::{WebOptions, WebRunner};
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[allow(unused_imports)]
use wasm_bindgen_futures::spawn_local;
#[allow(unused_imports)]
use web_sys::js_sys::Promise;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn openDirectoryPicker() -> Promise;
}

#[cfg(target_arch = "wasm32")]
pub fn start_game(
    canvas: web_sys::HtmlCanvasElement,
    init: AppCreator<'static>,
) -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let web_options = WebOptions::default();
    spawn_local(async move {
        WebRunner::new()
            .start(canvas, web_options, init)
            .await
            .expect("Failed to start eframe");
    });
    Ok(())
}

/* TODO implement right-click with popup when this
    https://github.com/emilk/egui/blob/master/crates/egui/src/containers/popup.rs
    gets into a proper egui release
*/
#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {{
        #[cfg(target_arch = "wasm32")]
        $crate::log(format!($($arg)*).as_str());
        #[cfg(not(target_arch = "wasm32"))]
        println!($($arg)*);
    }};
}
