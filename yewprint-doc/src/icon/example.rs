use yew::prelude::*;
use yewprint::{Icon, IconName};

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Example {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Icon icon={IconName::Print} />
            </div>
        }
    }
}
