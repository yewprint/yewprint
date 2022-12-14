use yew::prelude::*;
use yewprint::{Icon, Intent};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub icon: Icon,
    pub intent: Option<Intent>,
    pub size: i32,
}

#[function_component(Example)]
pub fn example(ExampleProps { icon, intent, size }: &ExampleProps) -> Html {
    html! {
        <div>
            <Icon {icon} {intent} {size} />
        </div>
    }
}
