mod app;
mod button_group;
mod buttons;
mod callout;
mod card;
mod collapse;
mod divider;
mod example;
mod html_select;
mod icon;
mod menu;
mod progressbar;
mod switch;
mod tabs;
mod tag;
mod text;
mod tree;

pub use app::*;
pub use example::*;

#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[macro_export]
macro_rules! include_raw_html {
    ($file:expr $(, $class:expr)?) => {{
        yew::virtual_dom::VNode::VRef(yew::web_sys::Node::from({
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(include_str!($file));
            $(div.set_class_name($class);)*
            div
        }))
    }};
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
