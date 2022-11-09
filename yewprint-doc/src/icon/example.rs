use yew::prelude::*;
use yewprint::{Icon, IconName, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub icon_name: IconName,
    pub intent: Option<Intent>,
    pub icon_size: i32,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div>
            <Icon
                icon={props.icon_name}
                intent={props.intent}
                icon_size={props.icon_size}
            />
        </div>
    }
}
