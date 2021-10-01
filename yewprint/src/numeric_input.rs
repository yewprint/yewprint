use crate::{Button, ButtonGroup, ControlGroup, IconName, InputGroup, Intent};
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::str::FromStr;
use yew::prelude::*;

pub struct NumericInput<T: Add + Clone + Display + FromStr + PartialEq + PartialOrd + Sub + 'static>
where
    <T as Sub>::Output: ToString,
    <T as Add>::Output: ToString,
{
    props: NumericInputProps<T>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NumericInputProps<
    T: Add + Clone + Display + FromStr + PartialEq + PartialOrd + Sub + 'static,
> where
    <T as Sub>::Output: ToString,
    <T as Add>::Output: ToString,
{
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    pub min_value: T,
    pub max_value: T,
    #[prop_or_default]
    pub value: String,
    pub range: T,
}

pub enum Msg {
    AddEntry,
    UpdateValue(String),
    Up,
    Down,
    Noop,
}

impl<T: Add + Clone + Display + FromStr + PartialEq + PartialOrd + Sub + 'static> Component
    for NumericInput<T>
where
    <T as Sub>::Output: ToString,
    <T as Add>::Output: ToString,
{
    type Message = Msg;
    type Properties = NumericInputProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => {
                Msg::UpdateValue(self.props.value.clone());
                true
            }
            Msg::UpdateValue(value) => {
                if let Ok(_num) = value.trim().parse::<T>() {
                    self.props.value = value;
                    true
                } else {
                    false
                }
            }
            Msg::Up => {
                if let Ok(num) = self.props.value.trim().parse::<T>() {
                    if num >= self.props.max_value {
                        self.props.value = self.props.max_value.to_string();
                        true
                    } else {
                        self.props.value = (num + self.props.range.clone()).to_string();
                        true
                    }
                } else {
                    self.props.value = self.props.max_value.to_string();
                    true
                }
            }
            Msg::Down => {
                if let Ok(num) = self.props.value.trim().parse::<T>() {
                    if num <= self.props.min_value {
                        self.props.value = self.props.min_value.to_string();
                        true
                    } else {
                        self.props.value = (num - self.props.range.clone()).to_string();
                        true
                    }
                } else {
                    self.props.value = self.props.min_value.to_string();
                    true
                }
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
            <ControlGroup
                class=classes!("bp3-numeric-input")
                fill=self.props.fill
                large=self.props.large
            >
                <InputGroup
                    placeholder="Enter a number..."
                    large=self.props.large
                    disabled=self.props.disabled
                    value=self.props.value.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::AddEntry } else { Msg::Noop }
                    })
                />
                <ButtonGroup vertical=true class=classes!("bp3-fixed")>
                    <Button
                        icon=IconName::ChevronUp
                        disabled=self.props.disabled
                        onclick=self.link.callback(|_| Msg::Up)
                    />
                    <Button
                        icon=IconName::ChevronDown
                        disabled=self.props.disabled
                        onclick=self.link.callback(|_| Msg::Down)
                    />
                </ButtonGroup>
            </ControlGroup>
        }
    }
}
