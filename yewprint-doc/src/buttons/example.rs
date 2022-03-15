use yew::prelude::*;
use yewprint::Button;

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
}

pub enum Msg {
    AddOne,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <Button
                        onclick={ctx.link().callback(|_| Msg::AddOne)}
                        minimal={ctx.props().minimal}
                        fill={ctx.props().fill}
                        small={ctx.props().small}
                        outlined={ctx.props().outlined}
                        loading={ctx.props().loading}
                        large={ctx.props().large}
                        active={ctx.props().active}
                        disabled={ctx.props().disabled}
                    >
                        {"Add 1"}
                    </Button>
                </div>
            </div>
        }
    }
}
