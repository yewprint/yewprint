use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub indeterminate_state: bool,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    html! {
        <label
            class={classes!(
                "bp3-control", "bp3-checkbox",
                props.disabled.then(|| "bp3-disabled"),
                props.inline.then(|| "bp3-inline"),
                props.large.then(|| "bp3-large")
            )}
        >
            <input
                type="checkbox"
                checked={props.checked}
                onchange={props.onchange.clone()}
                disabled={props.disabled}
            />
            <span
                class="bp3-control-indicator"
            >
            </span>
            {props.label.clone()}
        </label>
    }
}
