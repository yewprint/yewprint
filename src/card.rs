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
pub fn card(
    CardProps {
        class,
        elevation,
        onclick,
        interactive,
        children,
    }: &CardProps,
) -> Html {
    html! {
        <div
            class={classes!(
                "bp3-card",
                elevation,
                interactive.then_some("bp3-interactive"),
                class.clone(),
            )}
            {onclick}
        >
            {children.clone()}
        </div>
    }
}
