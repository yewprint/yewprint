use yew::prelude::*;
use yewprint::{Icon, IconName, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div>
            <Icon
                icon={IconName::Print}
                intent={props.intent}
            />
        </div>
    }
}
