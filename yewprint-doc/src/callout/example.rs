use yew::prelude::*;
use yewprint::{Callout, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub show_icon: bool,
    pub show_title: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <Callout
            title={props.show_title.then(|| "Visually important content")}
            without_icon={!props.show_icon}
            intent={props.intent}
        >
            <p>{"The Callout element's background reflects its intent, if any."}</p>
        </Callout>
    }
}
