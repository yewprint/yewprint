use crate::Elevation;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub elevation: Elevation,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or(false)]
    pub interactive: bool,
    #[prop_or_default]
    pub style: Option<AttrValue>,
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
            // TODO why do I need to unwrap()? looks like an issue in Yew
            onclick={onclick.clone().unwrap_or_default()}
            {style}
        >
            {children.clone()}
        </div>
    }
}
