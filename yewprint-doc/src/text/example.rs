use yew::prelude::*;
use yewprint::Text;

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub ellipsize: bool,
    pub text: String,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="width: 150px; height: 20px">
                <Text ellipsize={ctx.props().ellipsize}>
                    {&ctx.props().text}
                </Text>
            </div>
        }
    }
}
