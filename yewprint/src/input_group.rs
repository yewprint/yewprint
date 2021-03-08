use crate::{Icon, IconName};
use yew::prelude::*;

pub struct InputGroup {
    props: InputGroupProps,
}

#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub enum TextInputType {
    Text,
    Password,
    Date,
    DateTime,
    Email,
    Month,
    Search,
    Telephone,
    Time,
    Url,
    Week,
}

impl TextInputType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Date => "date",
            Self::DateTime => "datetime-local",
            Self::Email => "email",
            Self::Month => "month",
            Self::Search => "search",
            Self::Telephone => "tel",
            Self::Time => "time",
            Self::Url => "url",
            Self::Week => "week",
        }
    }
}

impl Default for TextInputType {
    fn default() -> Self {
        Self::Text
    }
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
    #[prop_or_default]
    pub input_type: TextInputType,
    #[prop_or_default]
    pub class: Classes,
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
                class=classes!(
                    "bp3-input-group",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.small.then(|| "bp3-small"),
                    self.props.round.then(|| "bp3-round"),
                    self.props.placeholder.clone(),
                    self.props.class.clone(),
                )
            >
                {
                    if let Some(left_element) = self.props.left_element.clone() {
                        html! {
                            <span class="bp3-input-left-container">
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
                    type=self.props.input_type.as_str()
                    placeholder=&self.props.placeholder
                    disabled=self.props.disabled
                />
                {
                    if let Some(right_element) = self.props.right_element.clone() {
                        html! {
                            <span class="bp3-input-action">
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
