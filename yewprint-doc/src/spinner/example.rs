use yew::prelude::*;
use yewprint::{Intent, Spinner};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub size: u32,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div>
            <Spinner
                size={props.size as f32}
                intent={props.intent}
            />
        </div>
    }
}
