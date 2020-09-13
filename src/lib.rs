#![recursion_limit = "512"]

mod app;
pub mod buttons;
pub mod collapse;
pub mod forms;
pub mod icon;
pub mod tree;

use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
