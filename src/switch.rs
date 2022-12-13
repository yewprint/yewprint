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
    pub label: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inner_label_checked: Option<AttrValue>,
    #[prop_or_default]
    pub inner_label: Option<AttrValue>,
    #[prop_or_default]
    pub align_right: bool,
}

#[function_component(Switch)]
pub fn switch(
    SwitchProps {
        checked,
        disabled,
        inline,
        large,
        onclick,
        label,
        class,
        inner_label_checked,
        inner_label,
        align_right,
    }: &SwitchProps,
) -> Html {
    let display_label = (inner_label.is_some() || inner_label_checked.is_some()).then(|| {
        let inner_label = inner_label.clone().unwrap_or_default();

        html! {
            <>
                <div class={classes!("bp3-control-indicator-child")}>
                    <div class={classes!("bp3-switch-inner-text")}>
                        {
                            if let Some(label_checked) = inner_label_checked.clone() {
                                label_checked
                            } else {
                                inner_label.clone()
                            }
                        }
                    </div>
                </div>
                <div class={classes!("bp3-control-indicator-child")}>
                    <div class={classes!("bp3-switch-inner-text")}>
                        {inner_label}
                    </div>
                </div>
            </>
        }
    });

    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-switch",
                disabled.then_some("bp3-disabled"),
                inline.then_some("bp3-inline"),
                large.then_some("bp3-large"),
                if *align_right {
                    "bp3-align-right"
                } else {
                    "bp3-align-left"
                },
                class.clone(),
            )}
        >
        <input
            type="checkbox"
            checked={*checked}
            {onclick}
            disabled={*disabled}
        />
        <span
            class={classes!("bp3-control-indicator")}
        >
            {display_label}
        </span>
        {label.clone()}
        </label>
    }
}
