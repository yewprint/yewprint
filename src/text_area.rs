use crate::Intent;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub grow_vertically: bool,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(TextArea)]
pub fn text_area(
    TextAreaProps {
        class,
        fill,
        grow_vertically,
        r#ref,
        intent,
        large,
        small,
        onchange,
    }: &TextAreaProps,
) -> Html {
    let oninput = {
        let grow_vertically = *grow_vertically;
        Callback::from(move |e: InputEvent| {
            if grow_vertically {
                let input = e
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
                if let Some(input) = input {
                    input
                        .style()
                        .set_property("height", &format!("{}px", input.scroll_height()))
                        .unwrap();
                }
            }
        })
    };
    html! {
        <textarea
            class={classes!(
                "bp3-input",
                intent,
                fill.then_some("bp3-fill"),
                small.then_some("bp3-small"),
                large.then_some("bp3-large"),
                class.clone(),
            )}
            ref={r#ref}
            {onchange}
            {oninput}
        />
    }
}
