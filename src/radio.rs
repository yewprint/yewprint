use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub value: Option<AttrValue>,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-radio",
                props.disabled.then_some("bp3-disabled"),
                props.inline.then_some("bp3-inline"),
                props.large.then_some("bp3-large"),
            )}
        >
            <input
                type="radio"
                onchange={props.onchange.clone()}
                disabled={props.disabled}
                // Clone because `into_prop_value()` take ownership of `self`
                value={props.value.clone()}
                checked={props.checked.unwrap_or(false)}
                // Clone because `into_prop_value()` take ownership of `self`
                name={props.name.clone()}
            />
            <span
                class={classes!("bp3-control-indicator")}
            >
            </span>
            {props.label.clone()}
        </label>
    }
}
