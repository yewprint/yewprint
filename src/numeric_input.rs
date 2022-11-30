use crate::{Button, ButtonGroup, ControlGroup, IconName, InputGroup, Intent};
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Bound, RangeBounds, Sub};
use std::str::FromStr;
use web_sys::HtmlInputElement;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Debug)]
pub struct NumericInput<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + 'static,
{
    input: String,
    phantom: PhantomData<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NumericInputProps<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + 'static,
{
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub left_element: Option<Html>,
    #[prop_or_default]
    pub right_element: Option<Html>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    pub value: T,
    #[prop_or_default]
    pub bounds: NumericInputRangeBounds<T>,
    pub increment: T,
    #[prop_or_default]
    pub disable_buttons: bool,
    #[prop_or_default]
    pub buttons_on_the_left: bool,
}

pub enum Msg {
    InputUpdate(String),
    Up,
    Down,
    Noop,
}

impl<T> Component for NumericInput<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Copy
        + Display
        + FromStr
        + PartialEq
        + PartialOrd
        + 'static,
{
    type Message = Msg;
    type Properties = NumericInputProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: Default::default(),
            phantom: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut update_value = |new_value| {
            let new_value = ctx.props().bounds.clamp(new_value, ctx.props().increment);

            if new_value != ctx.props().value {
                self.input = new_value.to_string();
                ctx.props().onchange.emit(new_value);
                true
            } else {
                false
            }
        };

        match msg {
            Msg::InputUpdate(new_value) => {
                if let Ok(new_value) = new_value.trim().parse::<T>() {
                    update_value(new_value)
                } else {
                    false
                }
            }
            Msg::Up => update_value(ctx.props().value + ctx.props().increment),
            Msg::Down => update_value(ctx.props().value - ctx.props().increment),
            Msg::Noop => false,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.input = ctx.props().value.to_string();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let NumericInputProps {
            value,
            increment,
            disabled,
            disable_buttons,
            buttons_on_the_left,
            ..
        } = *ctx.props();

        let bounds = &ctx.props().bounds;
        let button_up_disabled = disabled || bounds.clamp(value + increment, increment) == value;
        let button_down_disabled = disabled || bounds.clamp(value - increment, increment) == value;

        let buttons = if disable_buttons {
            html!()
        } else {
            html! {
                <ButtonGroup vertical=true class={classes!("bp3-fixed")}>
                    <Button
                        icon={IconName::ChevronUp}
                        disabled={button_up_disabled}
                        onclick={ctx.link().callback(|_| Msg::Up)}
                    />
                    <Button
                        icon={IconName::ChevronDown}
                        disabled={button_down_disabled}
                        onclick={ctx.link().callback(|_| Msg::Down)}
                    />
                </ButtonGroup>
            }
        };

        let input_group = html! {
            <InputGroup
                placeholder={ctx.props().placeholder.clone()}
                large={ctx.props().large}
                disabled={ctx.props().disabled}
                left_icon={ctx.props().left_icon}
                left_element={ctx.props().left_element.clone()}
                right_element={ctx.props().right_element.clone()}
                value={self.input.clone()}
                oninput={ctx.link().callback(|e: InputEvent| {
                    let value = e.target_unchecked_into::<HtmlInputElement>().value();
                    Msg::InputUpdate(value)
                })}
                onkeydown={ctx.link().callback(|e: KeyboardEvent| {
                    if e.key() == "ArrowUp" {
                        Msg::Up
                    } else if e.key() == "ArrowDown" {
                        Msg::Down
                    } else {
                        Msg::Noop
                    }
                })}
            />
        };

        if buttons_on_the_left {
            html! {
                <ControlGroup
                    class={classes!("bp3-numeric-input")}
                    fill={ctx.props().fill}
                    large={ctx.props().large}
                >
                    {buttons}
                    {input_group}
                </ControlGroup>
            }
        } else {
            html! {
                <ControlGroup
                    class={classes!("bp3-numeric-input")}
                    fill={ctx.props().fill}
                    large={ctx.props().large}
                >
                    {input_group}
                    {buttons}
                </ControlGroup>
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumericInputRangeBounds<T> {
    pub start: Bound<T>,
    pub end: Bound<T>,
}

impl<T> Default for NumericInputRangeBounds<T> {
    fn default() -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl<T> NumericInputRangeBounds<T>
where
    T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn clamp(&self, value: T, increment: T) -> T {
        match (self.start, self.end) {
            (Bound::Included(min), _) if value < min => min,
            (Bound::Excluded(min), _) if value <= min => min + increment,
            (_, Bound::Included(max)) if value > max => max,
            (_, Bound::Excluded(max)) if value >= max => max - increment,
            _ => value,
        }
    }
}

macro_rules! impl_into_prop_value {
    ($path:path) => {
        impl<T: Copy> IntoPropValue<NumericInputRangeBounds<T>> for $path {
            fn into_prop_value(self) -> NumericInputRangeBounds<T> {
                NumericInputRangeBounds {
                    start: self.start_bound().cloned(),
                    end: self.end_bound().cloned(),
                }
            }
        }
    };
}

impl_into_prop_value!(std::ops::Range<T>);
impl_into_prop_value!(std::ops::RangeFrom<T>);
impl_into_prop_value!(std::ops::RangeFull);
impl_into_prop_value!(std::ops::RangeInclusive<T>);
impl_into_prop_value!(std::ops::RangeTo<T>);
impl_into_prop_value!(std::ops::RangeToInclusive<T>);
