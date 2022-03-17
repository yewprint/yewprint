use yew::prelude::*;
use yewprint::{Icon, IconName};

#[function_component(Example)]
pub fn example() -> Html {
    html! {
        <div>
            <Icon icon={IconName::Print} />
        </div>
    }
}
