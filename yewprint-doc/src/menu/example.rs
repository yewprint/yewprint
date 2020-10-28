use yew::prelude::*;
use yewprint::{IconName, Menu, MenuDivider, MenuItem};

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
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
                        text={html!("Custom SVG icon")}
                        // add the yewprint icon
                    />
                    <MenuDivider />
                    <MenuItem
                        text={html!{"New text box"}}
                        icon=IconName::NewTextBox
                    />
                    <MenuItem
                        text={html!{"New object"}}
                        icon=IconName::NewObject
                    />
                    <MenuItem
                        text={html!{"New link"}}
                        icon=IconName::NewLink
                    />
                    <MenuDivider />
                    <MenuItem
                        text={html!{"Settings"}}
                        icon=IconName::Cog
                        // add the label icon
                    />
                </Menu>
            </div>
        }
    }
}
