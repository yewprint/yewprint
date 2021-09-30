use crate::icon::ICON_SIZE_LARGE;
use crate::{Icon, IconName, Intent};
use yew::prelude::*;

pub struct Callout {
    props: CalloutProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CalloutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub without_icon: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: Option<String>,
    pub children: html::Children,
}

impl Component for Callout {
    type Message = ();
    type Properties = CalloutProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            return false;
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let icon = if self.props.without_icon {
            None
        } else {
            self.props.icon.or_else(|| {
                self.props.intent.map(|intent| match intent {
                    Intent::Primary => IconName::InfoSign,
                    Intent::Success => IconName::Tick,
                    Intent::Warning => IconName::WarningSign,
                    Intent::Danger => IconName::Error,
                })
            })
        };
        let classes = classes!(
            self.props.class.clone(),
            "bp3-callout",
            icon.map(|_| "bp3-callout-icon"),
            self.props.intent,
        );
        html! {
            <div class=classes>
                {
                    icon.iter()
                        .map(|name| html!{<Icon icon=name icon_size=ICON_SIZE_LARGE/>})
                        .collect::<Html>()
                }
                {
                    self.props.title.iter()
                        .map(|title| html!{<h4 class={"bp3-heading"}>{title}</h4>})
                        .collect::<Html>()
                }
                { self.props.children.clone() }
            </div>
        }
    }
}
