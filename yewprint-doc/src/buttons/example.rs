use yew::prelude::*;
use yewprint::Button;

pub struct Example {
    link: &html::Scope<Self>,
    counter: i64,
    props: ExampleProps,
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
        Example {
            counter: 0,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <Button
                        onclick={self.link.callback(|_| Msg::AddOne)}
                        minimal={self.props.minimal}
                        fill={self.props.fill}
                        small={self.props.small}
                        outlined={self.props.outlined}
                        loading={self.props.loading}
                        large={self.props.large}
                        active={self.props.active}
                        disabled={self.props.disabled}
                    >
                        {"Add 1"}
                    </Button>
                </div>
            </div>
        }
    }
}
