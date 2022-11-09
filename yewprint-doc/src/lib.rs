#![allow(
    clippy::redundant_closure,
    clippy::needless_update,
    dead_code,
    clippy::derive_partial_eq_without_eq,
    // Shadows lints linked to this issue: https://github.com/yewstack/yew/issues/2931
    clippy::let_unit_value
)]

mod app;
mod button_group;
mod buttons;
mod callout;
mod card;
mod checkbox;
mod collapse;
mod control_group;
mod divider;
mod example;
mod html_select;
mod icon;
mod input_group;
mod logo;
mod menu;
mod numeric_input;
mod panel_stack;
mod progress_bar;
mod radio;
mod slider;
mod spinner;
mod switch;
mod tabs;
mod tag;
mod text;
mod text_area;
mod tree;

pub use app::*;
pub use example::*;
pub use logo::*;

#[macro_export]
macro_rules! include_raw_html {
    ($file:expr $(, $class:expr)?) => {{
        yew::virtual_dom::VNode::VRef(web_sys::Node::from({
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
    () => {
        pub struct SourceCodeUrl {
            url: String,
        }

        impl SourceCodeUrl {
            pub fn generate_url() -> String {
                use std::path::Path;

                let component_name = Path::new(file!())
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                format!(
                    "https://github.com/yewprint/yewprint/blob/HEAD/src/{}.rs",
                    component_name,
                )
            }
        }

        impl Component for SourceCodeUrl {
            type Message = ();
            type Properties = ();

            fn create(_ctx: &Context<Self>) -> Self {
                let url = SourceCodeUrl::generate_url();

                Self { url }
            }

            fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
                true
            }

            fn view(&self, _ctx: &Context<Self>) -> Html {
                use yewprint::Text;

                html! {
                    <a
                        class={classes!("bp3-text-muted")}
                        href={self.url.clone()}
                        target="_blank"
                    >
                        <Text>{"Go to source code"}</Text>
                    </a>
                }
            }
        }

        #[cfg(test)]
        mod source_tests {
            use super::*;

            #[test]
            fn check_url() {
                let url = SourceCodeUrl::generate_url();
                let response = ureq::get(&url).call().expect("can send request");

                assert_eq!(response.status(), 200)
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
    yew::start_app::<app::AppRoot>();

    Ok(())
}
