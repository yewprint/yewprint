use yew::prelude::*;
use yewprint::{Button, Intent};

pub struct Example {
    counter: i64,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub small: bool,
    pub outlined: bool,
    pub loading: bool,
    pub large: bool,
    pub active: bool,
    pub disabled: bool,
    pub intent: Option<Intent>,
}

pub enum Msg {
    AddOne,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Example { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            minimal,
            fill,
            small,
            outlined,
            loading,
            large,
            active,
            disabled,
            intent,
        } = &ctx.props();

        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <Button
                        onclick={ctx.link().callback(|_| Msg::AddOne)}
                        {minimal}
                        {fill}
                        {small}
                        {outlined}
                        {loading}
                        {large}
                        {active}
                        {disabled}
                        {intent}
                    >
                        {"Add 1"}
                    </Button>
                </div>
            </div>
        }
    }
}
