use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub label: Html,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Radio)]
pub fn radio(
    RadioProps {
        disabled,
        inline,
        large,
        checked,
        name,
        onchange,
        label,
        value,
        class,
    }: &RadioProps,
) -> Html {
    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-radio",
                disabled.then_some("bp3-disabled"),
                inline.then_some("bp3-inline"),
                large.then_some("bp3-large"),
                class.clone(),
            )}
        >
            <input
                type="radio"
                {onchange}
                disabled={*disabled}
                {value}
                checked={*checked}
                {name}
            />
            <span
                class={classes!("bp3-control-indicator")}
            >
            </span>
            {label.clone()}
        </label>
    }
}
