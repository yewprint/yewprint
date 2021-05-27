use crate::app::DocMenu;
use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Icon, IconName, Menu, MenuDivider, MenuItem};

pub struct Example {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GoToMenu(DocMenu),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuItem
                        icon=IconName::NewObject
                        text={html!{"New object"}}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuItem
                        icon=IconName::NewLink
                        text={html!{"New link"}}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuDivider />
                    <MenuItem
                        icon=IconName::Cog
                        text={html!{"Settings"}}
                        label=html! {
                            <Icon icon=IconName::Share />
                        }
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                </Menu>
                <Menu>
                    <MenuDivider title={html!("Edit")} />
                    <MenuItem
                        icon=IconName::Cut
                        text={html!("Cut")}
                        label={html!("Ctrl+X")}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuItem
                        icon=IconName::Duplicate
                        text={html!("Copy")}
                        label={html!("Ctrl+C")}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
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
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuItem
                        icon=IconName::Style
                        text={html!("Style")}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                    <MenuItem
                        icon=IconName::Asterisk
                        text={html!("Miscellaneous")}
                        href=Cow::Borrowed("#menu")
                        onclick=self.link
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                    />
                </Menu>
            </>
        }
    }
}
