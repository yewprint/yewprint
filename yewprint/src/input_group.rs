use crate::{Icon, IconName};
use yew::prelude::*;

const MIN_HORIZONTAL_PADDING: i32 = 10;

pub struct InputGroup {
    props: InputGroupProps,
    link: &html::Scope<Self>,
    left_element_ref: NodeRef,
    left_element_width: Option<i32>,
    right_element_ref: NodeRef,
    right_element_width: Option<i32>,
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
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub onkeyup: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub input_ref: NodeRef,
}

impl Component for InputGroup {
    type Message = ();
    type Properties = InputGroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props(),
            link: ctx.link(),
            left_element_ref: Default::default(),
            left_element_width: Default::default(),
            right_element_ref: Default::default(),
            right_element_width: Default::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        let input_style = match (self.left_element_width, self.right_element_width) {
            (Some(left), None) => format!("padding-left:{}px", left.max(MIN_HORIZONTAL_PADDING)),
            (None, Some(right)) => format!("padding-right:{}px", right.max(MIN_HORIZONTAL_PADDING)),
            (Some(left), Some(right)) => format!(
                "padding-left:{}px;padding-right:{}px",
                left.max(MIN_HORIZONTAL_PADDING),
                right.max(MIN_HORIZONTAL_PADDING)
            ),
            _ => Default::default(),
        };

        html! {
            <div
                class={classes!(
                    "bp3-input-group",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.small.then(|| "bp3-small"),
                    self.props.round.then(|| "bp3-round"),
                    self.props.placeholder.clone(),
                    self.props.class.clone(),
                )}
            >
                {
                    if let Some(left_element) = self.props.left_element.clone() {
                        html! {
                            <span
                                class="bp3-input-left-container"
                                ref={self.left_element_ref.clone()}
                            >
                                {left_element}
                            </span>
                        }
                    } else if let Some(icon) = self.props.left_icon {
                        html! {
                            <Icon icon={icon} />
                        }
                    } else {
                        html!()
                    }
                }
                <input
                    ref={self.props.input_ref.clone()}
                    class="bp3-input"
                    type={self.props.input_type.as_str()}
                    placeholder={self.props.placeholder.clone()}
                    disabled={self.props.disabled}
                    oninput={self.props.oninput.clone()}
                    onkeyup={self.props.onkeyup.clone()}
                    onkeydown={self.props.onkeydown.clone()}
                    value={self.props.value.clone()}
                    style={input_style}
                />
                {
                    if let Some(right_element) = self.props.right_element.clone() {
                        html! {
                            <span
                                class="bp3-input-action"
                                ref={self.right_element_ref.clone()}
                            >
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

    fn rendered(&mut self, _first_render: bool) {
        let left_old_value = self.left_element_width.take();
        self.left_element_width = self
            .left_element_ref
            .cast::<web_sys::Element>()
            .map(|x| x.client_width());

        let right_old_value = self.right_element_width.take();
        self.right_element_width = self
            .right_element_ref
            .cast::<web_sys::Element>()
            .map(|x| x.client_width());

        if left_old_value != self.left_element_width || right_old_value != self.right_element_width
        {
            self.link.send_message(());
        }
    }
}
