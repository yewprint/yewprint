#![allow(
    clippy::redundant_closure,
    clippy::needless_update,
    dead_code,
    clippy::derive_partial_eq_without_eq,
    // Shadows lints linked to this issue: https://github.com/yewstack/yew/issues/2931
    clippy::let_unit_value,
    clippy::uninlined_format_args
)]

mod alert;
mod app;
mod button_group;
mod buttons;
mod callout;
mod card;
mod checkbox;
mod collapse;
mod control_group;
mod dialog;
mod divider;
mod example;
mod html_select;
mod icon;
mod input_group;
mod logo;
mod menu;
mod numeric_input;
mod overlay;
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
        Html::VRef(web_sys::Node::from({
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

                let component = Path::new(file!())
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                if let (Some(repo), Some(branch)) = (
                    option_env!("GITHUB_PR_SOURCE_REPOSITORY").filter(|x| !x.is_empty()),
                    option_env!("GITHUB_HEAD_REF").filter(|x| !x.is_empty()),
                ) {
                    format!("https://github.com/{repo}/blob/{branch}/src/{component}.rs")
                } else {
                    format!("https://github.com/yewprint/yewprint/blob/HEAD/src/{component}.rs")
                }
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
                assert!(ureq::get(&url).call().is_ok(), "could not get url: {url}");
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

    yewprint::Dark.auto_detect();
    yew::Renderer::<app::AppRoot>::new().render();

    Ok(())
}
