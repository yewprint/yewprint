use yew::prelude::*;
use yewprint::{Card, Elevation};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub elevation: Elevation,
    pub interactive: bool,
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Card elevation={ctx.props().elevation} interactive={ctx.props().interactive}>
                <p>
                    {
                        "This is a card component. The elevation of the card can be adjusted. \
                        An interactive card reacts to being moused over."
                    }
                </p>
            </Card>
        }
    }
}
