use boolinator::Boolinator;
use yew::prelude::*;
use crate::{Icon, IconName};

pub struct InputGroup {
    props: InputGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub lefticon: Option<IconName>,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for InputGroup {
    type Message = ();
    type Properties = InputGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        html! {
            <div
                class=(
                    "bp3-input-group",
                    self.props.disabled.as_some("bp3-disabled"),
                    self.props.large.as_some("bp3-large"),
                    self.props.small.as_some("bp3-small"),
                )
            >
                {
                    if let Some(icon) = self.props.lefticon {
                        html! {
                            <Icon icon=icon />
                        }
                    } else {
                        html!()
                    }
                }
                {self.props.children.clone()}
            </div>
        }
    }
}
