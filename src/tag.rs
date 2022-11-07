use crate::{Icon, IconName, Intent, Text};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct TagProps {
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    // FIXME Not clear that this field has any effect without `interactive` on.
    pub active: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub interactive: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub multiline: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onremove: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub right_icon: Option<IconName>,
    #[prop_or_default]
    pub round: bool,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let icon = props.icon.map(|icon| {
        html! {
            <Icon icon={icon} />
        }
    });

    let right_icon = props.right_icon.map(|icon| {
        html! {
            <Icon icon={icon} />
        }
    });

    let remove_button = props.onremove.clone().map(|callback| {
        html! {
            <button
                class={classes!("bp3-tag-remove")}
                onclick={callback}
                tabindex={props.interactive.then_some("0")}
            >
                <Icon icon={IconName::SmallCross} />
            </button>
        }
    });

    html! {
        <span
            class={classes!(
                "bp3-tag",
                props.intent,
                props.active.then_some("bp3-active"),
                props.fill.then_some("bp3-fill"),
                props.interactive.then_some("bp3-interactive"),
                props.large.then_some("bp3-large"),
                props.minimal.then_some("bp3-minimal"),
                props.round.then_some("bp3-round"),
                props.class.clone(),
            )}
            style={props.style.clone()}
            onclick={props.onclick.clone()}
        >
            {icon.unwrap_or_default()}
            <Text
                class={classes!("bp3-fill")}
                ellipsize={!props.multiline}
                title={props.title.clone()}
                inline=true
            >
                {props.children.clone()}
            </Text>
            {right_icon.unwrap_or_default()}
            {remove_button.unwrap_or_default()}
        </span>
    }
}
