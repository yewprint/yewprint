#![recursion_limit = "512"]

#[cfg(feature = "doc")]
extern crate self as yewprint;

#[cfg(feature = "doc")]
mod app;
mod buttons;
mod collapse;
#[cfg(feature = "doc")]
mod example;
mod icon;
mod menu;
mod switch;
mod tree;

#[cfg(feature = "doc")]
pub use app::*;
pub use buttons::*;
pub use collapse::*;
#[cfg(feature = "doc")]
pub use example::*;
pub use icon::*;
pub use id_tree;
pub use menu::*;
pub use switch::*;
pub use tree::*;

use std::ops::{Deref, DerefMut};
use yew::virtual_dom::{Classes, Transformer, VComp};

#[cfg(feature = "doc")]
#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[cfg(feature = "doc")]
#[macro_export]
macro_rules! include_raw_html {
    ($file:expr, $class:expr) => {{
        yew::virtual_dom::VNode::VRef(yew::web_sys::Node::from({
            let div = crate::include_raw_html!(element $file);
            div.set_class_name($class);
            div
        }))
    }};
    ($file:expr) => {{
        yew::virtual_dom::VNode::VRef(yew::web_sys::Node::from({
            crate::include_raw_html!(element $file)
        }))
    }};
    (element $file:expr) => {{
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(include_str!($file));
        div
    }};
}

// NOTE: this class needs to become deprecated when the feature bool_to_option lands in stable
//
//       https://github.com/rust-lang/rust/issues/64260
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ConditionalClass(bool);

impl Transformer<bool, ConditionalClass> for VComp {
    fn transform(value: bool) -> ConditionalClass {
        ConditionalClass(value)
    }
}

impl Transformer<ConditionalClass, bool> for VComp {
    fn transform(value: ConditionalClass) -> bool {
        value.0
    }
}

impl From<bool> for ConditionalClass {
    fn from(value: bool) -> Self {
        ConditionalClass(value)
    }
}

impl ConditionalClass {
    pub fn map_some(&self, value: &'static str) -> Option<&'static str> {
        if self.0 {
            Some(value)
        } else {
            None
        }
    }
}

impl Deref for ConditionalClass {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConditionalClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "doc")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Intent {
    Primary,
    Success,
    Warning,
    Danger,
}

impl From<Intent> for Classes {
    fn from(intent: Intent) -> Self {
        Classes::from(intent.as_static_str())
    }
}

impl Intent {
    fn as_static_str(&self) -> &'static str {
        match self {
            Intent::Primary => "bp3-intent-primary",
            Intent::Success => "bp3-intent-success",
            Intent::Warning => "bp3-intent-warning",
            Intent::Danger => "bp3-intent-danger",
        }
    }
}
