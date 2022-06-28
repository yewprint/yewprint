use crate::Elevation;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub elevation: Elevation,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub interactive: bool,
    pub children: html::Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!(
            "bp3-card",
            props.class.clone(),
            props.elevation,
            props.interactive.then(|| "bp3-interactive"),
        )}
        onclick={props.onclick.clone()}>
            {props.children.clone()}
        </div>
    }
}
