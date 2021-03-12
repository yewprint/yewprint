use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    histogram_value: String,
    password_value: String,
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
    Nope,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            link,
            histogram_value: String::new(),
            password_value: String::new(),
            tags_value: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::AddHistogramEntry => {
                yew::services::DialogService::alert(
                    format!("You sent: {}", self.histogram_value).as_str(),
                );
                self.histogram_value = String::new();
                true
            }
            Msg::UpdateHistogram(val) => {
                self.histogram_value = val;
                true
            }
            Msg::AddPasswordEntry => {
                yew::services::DialogService::alert(
                    format!("You sent: {}", self.password_value).as_str(),
                );
                self.password_value = String::new();
                true
            }
            Msg::UpdatePassword(val) => {
                self.password_value = val;
                true
            }
            Msg::AddTagsEntry => {
                yew::services::DialogService::alert(
                    format!("You sent: {}", self.tags_value).as_str(),
                );
                self.tags_value = String::new();
                true
            }
            Msg::UpdateTags(val) => {
                self.tags_value = val;
                true
            }
            Msg::Nope => false,
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
                    value=&self.histogram_value
                    oninput=self.link.callback(|e: InputData| Msg::UpdateHistogram(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddHistogramEntry } else { Msg::Nope }
                    })
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    placeholder={"Enter your password..."}
                    value=&self.password_value
                    oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddPasswordEntry } else { Msg::Nope }
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
                    value=&self.tags_value
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTags(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddTagsEntry } else { Msg::Nope }
                    })
                    right_element=html! {
                        <Tag
                            minimal=true
                            round=self.props.round
                        >
                            {"10000"}
                        </Tag>
                    }
                />
            </>
        }
    }
}
