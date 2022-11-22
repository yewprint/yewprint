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
    #[prop_or_default]
    pub inner_label_checked: Option<String>,
    #[prop_or_default]
    pub inner_label: Option<String>,
    #[prop_or_default]
    pub align_right: bool,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let display_label = {
        if props.inner_label.is_some() || props.inner_label_checked.is_some() {
            let inner_label = props.inner_label.as_deref().unwrap_or_default();
            let inner_label_checked = props.inner_label_checked.as_ref();
            html! {
                <>
                    <div class={classes!("bp3-control-indicator-child")}>
                        <div class={classes!("bp3-switch-inner-text")}>
                            {
                                if let Some(label_checked) = inner_label_checked {
                                    label_checked.clone()
                                } else {
                                    inner_label.to_string()
                                }
                            }
                        </div>
                    </div>
                    <div class={classes!("bp3-control-indicator-child")}>
                        <div class={classes!("bp3-switch-inner-text")}>
                            {inner_label.to_string()}
                        </div>
                    </div>
                </>
            }
        } else {
            Html::default()
        }
    };
    html! {
        <label
            class={classes!(
                "bp3-control",
                "bp3-switch",
                props.disabled.then_some("bp3-disabled"),
                props.inline.then_some("bp3-inline"),
                props.large.then_some("bp3-large"),
                props.class.clone(),
                if props.align_right {
                    "bp3-align-right"
                } else {
                    "bp3-align-left"
                },
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
            {display_label}
        </span>
        {props.label.clone()}
        </label>
    }
}
