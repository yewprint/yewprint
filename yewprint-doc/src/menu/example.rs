use yew::prelude::*;
use yewprint::{Menu, MenuItem};

pub struct Example {
    link: ComponentLink<Self>,
}

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Menu>
                    <MenuItem />
                </Menu>
            </div>
        }
    }
}