use crate::{Button, ConditionalClass, Icon, IconName, Intent, Text};
use yew::prelude::*;

pub struct Tag {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: html::Children,
    #[prop_or_default]
    // FIXME Not clear that this field has any effect without `interactive` on.
    pub active: ConditionalClass,
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub interactive: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub multiline: ConditionalClass,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub onremove: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub right_icon: Option<IconName>,
    #[prop_or_default]
    pub round: ConditionalClass,
    #[prop_or_default]
    pub title: Option<String>,
}

impl Component for Tag {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tag { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let icon = if let Some(icon) = self.props.icon {
            html!(<Icon icon=icon />)
        } else {
            html!()
        };

        let right_icon = if let Some(right_icon) = self.props.right_icon {
            html!(<Icon icon=right_icon />)
        } else {
            html!()
        };

        let remove_button = if let Some(callback) = self.props.onremove.clone() {
            html!(
                <Button
                    class="bp3-tag-remove"
                    onclick={callback.clone()}
                >
                    <Icon icon=IconName::SmallCross />
                </Button>
            )
        } else {
            html!()
        };

        html! {
            <span
                class=(
                    "bp3-tag",
                    self.props.intent,
                    self.props.active.map_some("bp3-active"),
                    self.props.fill.map_some("bp3-fill"),
                    self.props.interactive.map_some("bp3-interactive"),
                    self.props.large.map_some("bp3-large"),
                    self.props.minimal.map_some("bp3-minimal"),
                    self.props.round.map_some("bp3-round"),
                )
                onclick={self.props.onclick.clone()}
            >
                {icon}
                <Text
                    class="bp3-fill"
                    ellipsize={!self.props.multiline}
                    title=self.props.title.clone()
                    inline=true
                >
                    {self.props.children.clone()}
                </Text>
                {remove_button}
                {right_icon}
            </span>
        }
    }
}
