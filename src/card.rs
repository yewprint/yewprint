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
    #[prop_or_default]
    pub style: AttrValue,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(
    CardProps {
        class,
        elevation,
        onclick,
        interactive,
        style,
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
            {style}
        >
            {children.clone()}
        </div>
    }
}
