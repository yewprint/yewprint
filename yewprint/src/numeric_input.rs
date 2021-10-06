use crate::{Button, ButtonGroup, ControlGroup, IconName, InputGroup, Intent};
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::str::FromStr;
use yew::prelude::*;

pub struct NumericInput<
    T: Add<Output = T>
        + Clone
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + 'static,
> where
    <T as Sub>::Output: ToString,
    <T as Add>::Output: ToString,
{
    props: NumericInputProps<T>,
    link: ComponentLink<Self>,
    input: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NumericInputProps<
    T: Add<Output = T>
        + Clone
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + Sub<Output = T>
        + 'static,
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
    pub placeholder: String,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: Option<T>,
    pub min_value: T,
    pub max_value: T,
    pub increment: T,
}

pub enum Msg {
    UpdateValue(String),
    Up,
    Down,
    Noop,
}

impl<
        T: Add<Output = T>
            + Clone
            + Display
            + FromStr
            + PartialEq
            + PartialOrd
            + Sub<Output = T>
            + 'static,
    > Component for NumericInput<T>
where
    <T as Sub>::Output: ToString,
    <T as Add>::Output: ToString,
{
    type Message = Msg;
    type Properties = NumericInputProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            input: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(value) => {
                if let Ok(num) = value.trim().parse::<T>() {
                    self.props.value = Some(num.clone());
                    self.input = num.to_string();
                    true
                } else {
                    false
                }
            }
            Msg::Up => {
                if let Some(num) = self.props.value.clone() {
                    if num >= self.props.max_value {
                        self.props.value = Some(self.props.max_value.clone());
                        self.input = self.props.max_value.to_string();
                        true
                    } else {
                        self.props.value = Some(num.clone() + self.props.increment.clone());
                        self.input = (num + self.props.increment.clone()).to_string();
                        true
                    }
                } else {
                    false
                }
            }
            Msg::Down => {
                if let Some(num) = self.props.value.clone() {
                    if num <= self.props.min_value {
                        self.props.value = Some(self.props.min_value.clone());
                        self.input = self.props.min_value.to_string();
                        true
                    } else {
                        self.props.value = Some(num.clone() - self.props.increment.clone());
                        self.input = (num - self.props.increment.clone()).to_string();
                        true
                    }
                } else {
                    false
                }
            }
            Msg::Noop => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            if self.props.value != props.value {
                if let Some(value) = props.value.as_ref() {
                    self.input = value.to_string();
                }
            }
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
                    placeholder=self.props.placeholder.clone()
                    large=self.props.large
                    disabled=self.props.disabled
                    left_icon=self.props.left_icon
                    value=self.input.clone()
                    oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                    onkeydown=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "ArrowUp" {
                            Msg::Up
                        } else if e.key() == "ArrowDown" {
                            Msg::Down
                        } else {
                            Msg::Noop
                        }
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
