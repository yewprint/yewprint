use yew::prelude::*;
use yewprint::{Intent, ProgressBar};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub animate: bool,
    pub stripes: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <ProgressBar
            animate={props.animate}
            stripes={props.stripes}
            intent={props.intent}
            value=30_u8
        />
    }
}
