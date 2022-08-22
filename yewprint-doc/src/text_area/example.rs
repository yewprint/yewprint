use yew::prelude::*;
use yewprint::{Intent, TextArea};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub small: bool,
    pub large: bool,
    pub fill: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div style="width: 200px; height: 50px">
            <TextArea intent={props.intent}
                      large={props.large}
                      fill={props.fill}
                      small={props.small}
            />
        </div>
    }
}
