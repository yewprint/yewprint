use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-switch",
                props.disabled.then(|| "bp3-disabled"),
                props.inline.then(|| "bp3-inline"),
                props.large.then(|| "bp3-large"),
                props.class.clone(),
            )}
        >
        <input
            type="checkbox"
            checked={props.checked}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        />
        <span
            class={classes!("bp3-control-indicator")}
        >
        </span>
        {props.label.clone()}
        </label>
    }
}
