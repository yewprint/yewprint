use crate::{Icon, IconName, Intent, Spinner, ICON_SIZE_LARGE};
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
    pub icon: Option<IconName>,
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
    pub children: html::Children,
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
        intent,
        title,
        onclick,
        class,
        style,
        children,
    } = props;

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
            onclick={(!disabled).then_some(onclick.clone())}
        >
            {
                loading
                    .then(|| html! {
                        <Spinner
                            class={classes!("bp3-button-spinner")}
                            size={ICON_SIZE_LARGE as f32}
                        />
                    })
            }
            {icon.map(|icon| html!(<Icon {icon} />))}
            {
                (!children.is_empty())
                    .then(|| html! {
                        <span class="bp3-button-text">
                            {for children.iter()}
                        </span>
                    })
            }
        </button>
    }
}
