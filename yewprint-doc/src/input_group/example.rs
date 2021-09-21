use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    histogram_value: String,
    password_value: String,
    password_strength: Html,
    tags_value: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub fill: bool,
    pub large: bool,
    pub small: bool,
    pub round: bool,
}

pub enum Msg {
    AddHistogramEntry,
    UpdateHistogram(String),
    AddPasswordEntry,
    UpdatePassword(String),
    AddTagsEntry,
    UpdateTags(String),
    Noop,
}

macro_rules! alert {
    ($($arg:tt)*) => {
        yew::services::DialogService::alert(&format!(
            $($arg)*
        ))
    };
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            link,
            histogram_value: Default::default(),
            password_value: Default::default(),
            password_strength: Default::default(),
            tags_value: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::AddHistogramEntry => {
                alert!("You sent: {}", self.histogram_value);
                self.histogram_value = Default::default();
                true
            }
            Msg::UpdateHistogram(value) => {
                self.histogram_value = value;
                true
            }
            Msg::AddPasswordEntry => {
                alert!("You sent: {}", self.password_value);
                self.password_value = Default::default();
                true
            }
            Msg::UpdatePassword(value) => {
                self.password_strength = match value.len() {
                    n if n == 0 => html!(),
                    n if n < 4 => html!(<Tag>{"weak"}</Tag>),
                    n if n < 8 => html!(<Tag>{"medium"}</Tag>),
                    _ => html!(<Tag>{"strong"}</Tag>),
                };

                self.password_value = value;

                true
            }
            Msg::AddTagsEntry => {
                alert!("You sent: {}", self.tags_value);
                self.tags_value = Default::default();
                true
            }
            Msg::UpdateTags(val) => {
                self.tags_value = val;
                true
            }
            Msg::Noop => false,
        }
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
            <>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    left_icon=IconName::Filter
                    placeholder={"Filter histogram..."}
                    value=self.histogram_value.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdateHistogram(e.value))
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddHistogramEntry } else { Msg::Noop }
                    })
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    left_element=self.password_strength.clone()
                    placeholder={"Enter your password..."}
                    value=self.password_value.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddPasswordEntry } else { Msg::Noop }
                    })
                    right_element=html! {
                        <Button
                            icon=IconName::Lock
                            minimal=true
                            disabled=self.props.disabled
                        />
                    }
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    left_icon=IconName::Tag
                    placeholder={"Find tags"}
                    value=self.tags_value.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTags(e.value))
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddTagsEntry } else { Msg::Noop }
                    })
                    right_element=html! {
                        <Tag
                            minimal=true
                            round=self.props.round
                        >
                            {(10000 / 1.max(self.tags_value.len().pow(2)))}
                        </Tag>
                    }
                />
            </>
        }
    }
}
