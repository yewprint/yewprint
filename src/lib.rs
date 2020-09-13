#![recursion_limit = "512"]

#[cfg(feature = "dev")]
mod app;
pub mod buttons;
pub mod collapse;
pub mod forms;
pub mod icon;
pub mod tree;

#[cfg(feature = "dev")]
#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[cfg(feature = "dev")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
