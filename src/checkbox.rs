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
    pub label: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Checkbox)]
pub fn checkbox(
    CheckboxProps {
        disabled,
        inline,
        large,
        checked,
        onchange,
        label,
        class,
        children,
    }: &CheckboxProps,
) -> Html {
    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-checkbox",
                disabled.then_some("bp3-disabled"),
                inline.then_some("bp3-inline"),
                large.then_some("bp3-large"),
                class.clone(),
            )}
        >
            <input type="checkbox" checked={*checked} {onchange} disabled={*disabled} />
            <span class="bp3-control-indicator" />
            {label.clone()}
            {children.clone()}
        </label>
    }
}
