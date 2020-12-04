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
            <>
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
                <Menu>
                    <MenuDivider title={html!("Edit")} />
                    <MenuItem 
                        text={html!("Cut")}
                        icon=IconName::Cut
                    />
                    <MenuItem
                        text={html!("Copy")}
                        icon=IconName::Duplicate
                    />
                    <MenuItem
                        text={html!("Paste")}
                        icon=IconName::Clipboard
                        disabled=true
                    />
                    <MenuDivider title={html!("Text")} />
                    <MenuItem
                        text={html!("Alignment")}
                        icon=IconName::AlignLeft
                        disabled=true
                    />
                    <MenuItem
                        text={html!("Style")}
                        icon=IconName::Style
                    />
                    <MenuItem
                        text={html!("Miscellaneous")}
                        icon=IconName::Asterisk
                    />
                </Menu>
            </>
        }
    }
}
