use crate::{if_html, Icon, IconName, Intent, Text};
use std::borrow::Cow;
use yew::prelude::*;

pub struct Tag {
    props: TagProps,
}

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
    pub title: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,
}

impl Component for Tag {
    type Message = ();
    type Properties = TagProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tag { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let icon = if_html!(let Some(icon) = self.props.icon => <Icon icon={icon} />);

        let right_icon =
            if_html!(let Some(right_icon) = self.props.right_icon => <Icon icon={right_icon} />);

        let remove_button = if_html! {
            let Some(callback) = self.props.onremove.clone() =>
            html!(
                <button
                    class={classes!("bp3-tag-remove")}
                    onclick={callback}
                    tabindex={self.props.interactive.then(|| "0")}
                >
                    <Icon icon={IconName::SmallCross} />
                </button>
            )
        };

        html! {
            <span
                class={classes!(
                    "bp3-tag",
                    self.props.intent,
                    self.props.active.then(|| "bp3-active"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.interactive.then(|| "bp3-interactive"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.round.then(|| "bp3-round"),
                    self.props.class.clone(),
                )}
                style={self.props.style.clone()}
                onclick={self.props.onclick.clone()}
            >
                {icon}
                <Text
                    class={classes!("bp3-fill")}
                    ellipsize={!self.props.multiline}
                    title={self.props.title.clone()}
                    inline=true
                >
                    {self.props.children.clone()}
                </Text>
                {right_icon}
                {remove_button}
            </span>
        }
    }
}
