use crate::{Icon, Intent, Text};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct TagProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    // FIXME Not clear that this field has any effect without `interactive` on.
    pub active: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
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
    pub right_icon: Option<Icon>,
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
pub fn tag(
    TagProps {
        children,
        active,
        fill,
        icon,
        intent,
        interactive,
        large,
        minimal,
        multiline,
        onclick,
        onremove,
        right_icon,
        round,
        title,
        class,
        style,
    }: &TagProps,
) -> Html {
    let remove_button = onremove.clone().map(|onclick| {
        html! {
            <button
                class={classes!("bp3-tag-remove")}
                {onclick}
                tabindex={interactive.then_some("0")}
            >
                <Icon icon={Icon::SmallCross} />
            </button>
        }
    });

    html! {
        <span
            class={classes!(
                "bp3-tag",
                intent,
                active.then_some("bp3-active"),
                fill.then_some("bp3-fill"),
                interactive.then_some("bp3-interactive"),
                large.then_some("bp3-large"),
                minimal.then_some("bp3-minimal"),
                round.then_some("bp3-round"),
                class.clone(),
            )}
            {style}
            {onclick}
        >
            <Icon {icon} />
            <Text
                class={classes!("bp3-fill")}
                ellipsize={!multiline}
                {title}
                inline=true
            >
                {children.clone()}
            </Text>
            <Icon icon={right_icon} />
            {remove_button}
        </span>
    }
}
