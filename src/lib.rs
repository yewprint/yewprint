mod app;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
