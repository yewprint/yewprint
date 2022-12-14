use yew::prelude::*;
use yewprint::{Callout, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub show_icon: bool,
    pub show_title: bool,
}

#[function_component(Example)]
pub fn example(
    ExampleProps {
        intent,
        show_icon,
        show_title,
    }: &ExampleProps,
) -> Html {
    html! {
        <Callout
            title={show_title.then_some("Visually important content")}
            without_icon={!show_icon}
            {intent}
        >
            <p>{"The Callout element's background reflects its intent, if any."}</p>
        </Callout>
    }
}
