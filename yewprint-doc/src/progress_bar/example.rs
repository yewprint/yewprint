use yew::prelude::*;
use yewprint::{Intent, ProgressBar};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub animate: bool,
    pub stripes: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }
    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <ProgressBar
                animate={self.props.animate}
                stripes={self.props.stripes}
                intent={self.props.intent}
                value=0.3
            />
        }
    }
}
