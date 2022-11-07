use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    html! {
        <div
            class={classes!(
                "bp3-button-group",
                props.minimal.then_some("bp3-minimal"),
                props.fill.then_some("bp3-fill"),
                props.large.then_some("bp3-large"),
                props.vertical.then_some("bp3-vertical"),
                props.class.clone(),
            )}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}
