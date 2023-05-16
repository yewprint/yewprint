use crate::{Icon, IconSize, Intent, Spinner};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

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
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub right_icon: Option<Icon>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub button_ref: NodeRef,
    #[prop_or_default]
    pub left_element: Option<Html>,
    #[prop_or_default]
    pub right_element: Option<Html>,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        fill,
        minimal,
        small,
        outlined,
        loading,
        large,
        active,
        disabled,
        icon,
        right_icon,
        intent,
        title,
        onclick,
        class,
        style,
        button_ref,
        left_element,
        right_element,
        aria_label,
        children,
    } = props;

    let disabled = *disabled || *loading;

    html! {
        <button
            class={classes!(
                "bp3-button",
                fill.then_some("bp3-fill"),
                minimal.then_some("bp3-minimal"),
                small.then_some("bp3-small"),
                outlined.then_some("bp3-outlined"),
                loading.then_some("bp3-loading"),
                large.then_some("bp3-large"),
                (*active && !disabled).then_some("bp3-active"),
                disabled.then_some("bp3-disabled"),
                intent,
                class.clone(),
            )}
            {style}
            {title}
            aria-label={aria_label}
            onclick={(!disabled).then_some(onclick.clone())}
            ref={button_ref.clone()}
        >
            {
                loading
                    .then(|| html! {
                        <Spinner
                            class={classes!("bp3-button-spinner")}
                            size={IconSize::LARGE}
                        />
                    })
            }
            <Icon {icon} />
            {left_element.clone()}
            {
                (!children.is_empty())
                    .then(|| html! {
                        <span class="bp3-button-text">
                            {for children.iter()}
                        </span>
                    })
            }
            <Icon icon={right_icon} />
            {right_element.clone()}
        </button>
    }
}
