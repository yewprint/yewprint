use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    entries: Vec<Entry>,
    value: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub fill: bool,
    pub large: bool,
    pub small: bool,
    pub round: bool,
}

#[derive(Debug)]
pub struct Entry(String);

pub enum Msg {
    AddEntry,
    Update(String),
    Nope,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = Vec::new();
        let value = String::new();
        Example {
            props,
            link,
            entries,
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => {
                let entry_value = self.value.trim();
                if !entry_value.is_empty() {
                    let entry = Entry(entry_value.to_string());
                    self.entries.push(entry);
                }
                self.value = String::new();
            }
            Msg::Update(val) => {
                self.value = val;
            }
            Msg::Nope => {}
        }
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
            <>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    left_icon=IconName::Filter
                    placeholder={"Filter histogram..."}
                    value=&self.value
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddEntry } else { Msg::Nope }
                    })
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    placeholder={"Enter your password..."}
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
