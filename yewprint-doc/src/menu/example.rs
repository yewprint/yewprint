use yew::prelude::*;
use yewprint::{Menu, MenuItem, MenuDivider, Icon, IconName};

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
                    <MenuItem
                        text={html!("Custom SVG Icon")}
                        // add the yewprint icon
                    />
                    <MenuDivider />
                </Menu>
            </div>
        }
    }
}