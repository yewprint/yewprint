use yew::prelude::*;

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
    pub style: Option<String>,
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
                props.minimal.then(|| "bp3-minimal"),
                props.fill.then(|| "bp3-fill"),
                props.large.then(|| "bp3-large"),
                props.vertical.then(|| "bp3-vertical"),
                props.class.clone(),
            )}
            style={props.style.clone()}
        >
            {props.children.clone()}
        </div>
    }
}
