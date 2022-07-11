use crate::{Icon, IconName, Intent, Spinner, ICON_SIZE_LARGE};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<&'static str>,
    #[prop_or_default]
    pub children: html::Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            class={classes!(
                "bp3-button",
                props.fill.then(|| "bp3-fill"),
                props.minimal.then(|| "bp3-minimal"),
                props.small.then(|| "bp3-small"),
                props.outlined.then(|| "bp3-outlined"),
                props.loading.then(|| "bp3-loading"),
                props.large.then(|| "bp3-large"),
                (props.active && !props.disabled).then(|| "bp3-active"),
                props.disabled.then(|| "bp3-disabled"),
                props.intent,
                props.class.clone(),
            )}
            style={props.style}
            onclick={(!props.disabled).then(|| props.onclick.clone())}
        >
            {
                props
                    .loading
                    .then(|| html! {
                        <Spinner
                            class={classes!("bp3-button-spinner")}
                            size={ICON_SIZE_LARGE as f32}
                        />
                    })
                    .unwrap_or_default()
            }
            {
                if let Some(icon) = props.icon {
                    html! {
                        <Icon icon={icon} />
                    }
                } else {
                    html!()
                }
            }
            {
                if props.children.is_empty() {
                    html! ()
                } else {
                    html! {
                        <span class="bp3-button-text">
                            {for props.children.iter()}
                        </span>
                    }
                }
            }
        </button>
    }
}
