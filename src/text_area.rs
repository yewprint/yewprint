use crate::Intent;
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
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
}

#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let classes = classes!(
        "bp3-input",
        props.intent,
        props.class.clone(),
        props.fill.then_some("bp3-fill"),
        props.small.then_some("bp3-small"),
        props.large.then_some("bp3-large"),
    );
    html! {
        <textarea
            class={classes}
            onchange={props.onchange.clone()}
        />
    }
}
