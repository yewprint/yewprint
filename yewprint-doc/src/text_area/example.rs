use yew::prelude::*;
use yewprint::{Intent, TextArea};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub small: bool,
    pub large: bool,
    pub fill: bool,
    pub grow_vertically: bool,
    pub text: String,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div style="width: 200px; min-height: 50px">
            <TextArea intent={props.intent}
                      large={props.large}
                      fill={props.fill}
                      small={props.small}
                      grow_vertically={props.grow_vertically}
                      value={Some(props.text.clone())}
            />
        </div>
    }
}
