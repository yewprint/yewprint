use gloo::dialogs::alert;
use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};

pub struct Example {
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

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            histogram_value: Default::default(),
            password_value: Default::default(),
            password_strength: Default::default(),
            tags_value: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddHistogramEntry => {
                alert(&format!("You sent: {}", self.histogram_value));
                self.histogram_value = Default::default();
                true
            }
            Msg::UpdateHistogram(value) => {
                self.histogram_value = value;
                true
            }
            Msg::AddPasswordEntry => {
                alert(&format!("You sent: {}", self.password_value));
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
                alert(&format!("You sent: {}", self.tags_value));
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <InputGroup
                    fill={ctx.props().fill}
                    large={ctx.props().large}
                    small={ctx.props().small}
                    round={ctx.props().round}
                    disabled={ctx.props().disabled}
                    left_icon={IconName::Filter}
                    placeholder={"Filter histogram..."}
                    value={self.histogram_value.clone()}
                    oninput={ctx.link().callback(|e: InputData| Msg::UpdateHistogram(e.value))}
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddHistogramEntry } else { Msg::Noop }
                    })}
                />
                <InputGroup
                    fill={ctx.props().fill}
                    large={ctx.props().large}
                    small={ctx.props().small}
                    round={ctx.props().round}
                    disabled={ctx.props().disabled}
                    left_element={self.password_strength.clone()}
                    placeholder={"Enter your password..."}
                    value={self.password_value.clone()}
                    oninput={ctx.link().callback(|e: InputData| Msg::UpdatePassword(e.value))}
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddPasswordEntry } else { Msg::Noop }
                    })}
                    right_element={{ html! {
                        <Button
                            icon={IconName::Lock}
                            minimal={true}
                            disabled={ctx.props().disabled}
                        />
                    }}}
                />
                <InputGroup
                    fill={ctx.props().fill}
                    large={ctx.props().large}
                    small={ctx.props().small}
                    round={ctx.props().round}
                    disabled={ctx.props().disabled}
                    left_icon={IconName::Tag}
                    placeholder={"Find tags"}
                    value={self.tags_value.clone()}
                    oninput={ctx.link().callback(|e: InputData| Msg::UpdateTags(e.value))}
                    onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddTagsEntry } else { Msg::Noop }
                    })}
                    right_element={{html! {
                        <Tag
                            minimal=true
                            round={ctx.props().round}
                        >
                            {{10000 / 1.max(self.tags_value.len().pow(2))}}
                        </Tag>
                    }}}
                />
            </>
        }
    }
}
