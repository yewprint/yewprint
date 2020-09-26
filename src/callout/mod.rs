#[cfg(feature = "doc")]
pub mod doc;

use crate::icon::SIZE_LARGE;
use crate::{Icon, IconName, Intent};
use yew::prelude::*;

pub struct Callout {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
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
    type Properties = Props;

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
            self.props
                .icon
                .or(self.props.intent.map(|intent| match intent {
                    Intent::Primary => IconName::InfoSign,
                    Intent::Success => IconName::Tick,
                    Intent::Warning => IconName::WarningSign,
                    Intent::Danger => IconName::Error,
                }))
        };
        let mut classes = Classes::from(self.props.class.clone()).extend("bp3-callout");
        if icon.is_some() {
            classes.push("bp3-callout-icon");
        }
        if let Some(ref intent) = self.props.intent {
            classes.push(intent.as_ref());
        }
        html! {
            <div class=classes>
                {
                    icon.iter()
                        .map(|name| html!{<Icon icon=name icon_size=SIZE_LARGE/>})
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
