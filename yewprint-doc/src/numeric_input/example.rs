use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Button, Callout, IconName, Intent, NumericInput};

pub struct Example {
    props: ExampleProps,
    link: &html::Scope<Self>,
    value: i32,
    value_two: i32,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
    pub disable_buttons: bool,
    pub buttons_on_the_left: bool,
    pub left_icon: bool,
}

pub enum Msg {
    Reset,
    UpdateValue(i32),
    UpdateValueTwo(i32),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            props,
            link,
            value: 0,
            value_two: 0,
        }
    }

    fn update(&mut self, _ctx:  &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.value = 4;
                self.value_two = 4;
            }
            Msg::UpdateValue(value) => {
                self.value = value;
            }
            Msg::UpdateValueTwo(value) => {
                self.value_two = value;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <NumericInput<i32>
                disabled={self.props.disabled}
                fill={self.props.large}
                value={self.value}
                bounds={-105..}
                increment=10
                placeholder={String::from("Greater or equal to -105...")}
                onchange={self.link.callback(|x| Msg::UpdateValue(x))}
                disable_buttons={self.props.disable_buttons}
                buttons_on_the_left={self.props.buttons_on_the_left}
                left_icon={self.props.left_icon.then(|| IconName::Dollar)}
            />
            <NumericInput<i32>
                disabled={self.props.disabled}
                fill={self.props.fill}
                large={self.props.large}
                value={self.value_two}
                bounds={-10..=10}
                increment=1
                placeholder={String::from("Integer between -10 and 10")}
                onchange={self.link.callback(|x| Msg::UpdateValueTwo(x))}
                disable_buttons={self.props.disable_buttons}
                buttons_on_the_left={self.props.buttons_on_the_left}
                left_icon={self.props.left_icon.then(|| IconName::Dollar)}
            />
            <Button
                icon={IconName::Refresh}
                onclick={self.link.callback(|_| Msg::Reset)}
            >
                {"Reset at 4"}
            </Button>
            <Callout
                title={Cow::Borrowed("Selected values")}
                intent={Intent::Primary}
            >
                <ul>
                    <li>{self.value}</li>
                    <li>{self.value_two}</li>
                </ul>
            </Callout>
            </>
        }
    }
}
