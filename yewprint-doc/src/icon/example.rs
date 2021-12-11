use yew::prelude::*;
use yewprint::{Icon, IconName};

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example {}
    }
    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <Icon icon=IconName::Print />
            </div>
        }
    }
}
