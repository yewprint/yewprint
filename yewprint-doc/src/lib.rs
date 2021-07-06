#![allow(clippy::redundant_closure, clippy::needless_update, dead_code)]

mod app;
mod button_group;
mod buttons;
mod callout;
mod card;
mod collapse;
mod control_group;
mod divider;
mod example;
mod html_select;
mod icon;
mod input_group;
mod logo;
mod menu;
mod panel_stack;
mod progress_bar;
mod radio;
mod slider;
mod spinner;
mod switch;
mod tabs;
mod tag;
mod text;
mod textarea;
mod tree;

pub use app::*;
pub use example::*;
pub use logo::*;

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

#[macro_export]
macro_rules! build_source_code_component {
    ($constant_name:ident, $url:expr, $test_name:ident) => {
        const $constant_name: &'static str = $url;

        pub struct SourceCodeUrl {
            props: SourceCodeUrlProps,
        }

        #[derive(Clone, PartialEq, Properties)]
        pub struct SourceCodeUrlProps {
            #[prop_or_default]
            pub url: &'static str,
        }

        impl Component for SourceCodeUrl {
            type Message = ();
            type Properties = SourceCodeUrlProps;

            fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self { props }
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                true
            }

            fn change(&mut self, props: Self::Properties) -> ShouldRender {
                if self.props != props {
                    self.props = props;
                    true
                } else {
                    false
                }
            }

            fn view(&self) -> Html {
                html! {
                    <a
                        class=classes!("bp3-text-muted")
                        href=self.props.url
                        target="_blank"
                    >
                        <Text>{"Go to the source code"}</Text>
                    </a>
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn $test_name() {
                let get_url = reqwest::blocking::get($constant_name).unwrap();

                assert!(get_url.status().is_success())
            }
        }
    };
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() -> Result<(), wasm_bindgen::JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    yew::start_app::<app::App>();

    Ok(())
}
