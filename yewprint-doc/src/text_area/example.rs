use yew::prelude::*;
use yewprint::{Intent, TextArea};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub small: bool,
    pub large: bool,
    pub fill: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="width: 200px; height: 50px">
                <TextArea intent={self.props.intent}
                          large={self.props.large}
                          fill={self.props.fill}
                          small={self.props.small}
                />
            </div>
        }
    }
}
