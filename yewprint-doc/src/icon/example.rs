use std::str::FromStr;
use yew::prelude::*;
use yewprint::{Icon, IconName, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub icon_name: String,
    pub intent: Option<Intent>,
    pub icon_size: i32,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    let icon_name = IconName::from_str(&props.icon_name).unwrap_or_default();

    html! {
        <div>
            <Icon
                icon={icon_name}
                intent={props.intent}
                icon_size={props.icon_size}
            />
        </div>
    }
}
