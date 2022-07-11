use crate::{if_html, Icon, IconName, Intent, Text};
use yew::prelude::*;

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
    pub title: Option<&'static str>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<&'static str>,
}

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let icon = if_html!(let Some(icon) = props.icon => <Icon icon={icon} />);

    let right_icon =
        if_html!(let Some(right_icon) = props.right_icon => <Icon icon={right_icon} />);

    let remove_button = if_html! {
        let Some(callback) = props.onremove.clone() =>
        html!(
            <button
                class={classes!("bp3-tag-remove")}
                onclick={callback}
                tabindex={props.interactive.then(|| "0")}
            >
                <Icon icon={IconName::SmallCross} />
            </button>
        )
    };

    html! {
        <span
            class={classes!(
                "bp3-tag",
                props.intent,
                props.active.then(|| "bp3-active"),
                props.fill.then(|| "bp3-fill"),
                props.interactive.then(|| "bp3-interactive"),
                props.large.then(|| "bp3-large"),
                props.minimal.then(|| "bp3-minimal"),
                props.round.then(|| "bp3-round"),
                props.class.clone(),
            )}
            style={props.style}
            onclick={props.onclick.clone()}
        >
            {icon}
            <Text
                class={classes!("bp3-fill")}
                ellipsize={!props.multiline}
                title={props.title}
                inline=true
            >
                {props.children.clone()}
            </Text>
            {right_icon}
            {remove_button}
        </span>
    }
}
