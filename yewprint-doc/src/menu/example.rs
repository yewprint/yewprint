use yew::prelude::*;
use yewprint::{Icon, IconName, Menu, MenuDivider, MenuItem};

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
                        icon=IconName::NewTextBox
                        text={html!{"New text box"}}
                    />
                    <MenuItem
                        icon=IconName::NewObject
                        text={html!{"New object"}}
                    />
                    <MenuItem
                        icon=IconName::NewLink
                        text={html!{"New link"}}
                    />
                    <MenuDivider />
                    <MenuItem
                        icon=IconName::Cog
                        text={html!{"Settings"}}
                        label=html! {
                            <Icon icon=IconName::Share />
                        }
                    />
                </Menu>
                <Menu>
                    <MenuDivider title={html!("Edit")} />
                    <MenuItem
                        icon=IconName::Cut
                        text={html!("Cut")}
                        label={html!("Ctrl+X")}
                    />
                    <MenuItem
                        icon=IconName::Duplicate
                        text={html!("Copy")}
                        label={html!("Ctrl+C")}
                    />
                    <MenuItem
                        icon=IconName::Clipboard
                        text={html!("Paste")}
                        label={html!("Ctrl+V")}
                        disabled=true
                    />
                    <MenuDivider title={html!("Text")} />
                    <MenuItem
                        icon=IconName::AlignLeft
                        text={html!("Alignment")}
                        disabled=false
                    />
                    <MenuItem
                        icon=IconName::Style
                        text={html!("Style")}

                    />
                    <MenuItem
                        icon=IconName::Asterisk
                        text={html!("Miscellaneous")}
                    />
                </Menu>
            </>
        }
    }
}
