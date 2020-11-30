use crate::{Icon, IconName};
use boolinator::Boolinator;
use yew::prelude::*;

pub struct InputGroup {
    props: InputGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub round: bool,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub left_element: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub right_element: Option<yew::virtual_dom::VNode>,

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
                    self.props.fill.as_some("bp3-fill"),
                    self.props.large.as_some("bp3-large"),
                    self.props.small.as_some("bp3-small"),
                    self.props.round.as_some("bp3-round"),
                    self.props.placeholder.clone()
                )
            >
                {
                    if let Some(left_element) = self.props.left_element.clone() {
                        html! {
                            <span disabled=self.props.disabled>
                                {left_element}
                            </span>
                        }
                    } else {
                        html!()
                    }
                }
                {
                    if let Some(icon) = self.props.left_icon {
                        html! {
                            <Icon icon=icon />
                        }
                    } else {
                        html!()
                    }
                }
                <input
                    class="bp3-input"
                    placeholder=&self.props.placeholder
                    disabled=self.props.disabled
                />
                {
                    if let Some(right_element) = self.props.right_element.clone() {
                        html! {
                            <span disabled=self.props.disabled>
                                {right_element}
                            </span>
                        }
                    } else {
                        html!()
                    }
                }
            </div>
        }
    }
}
