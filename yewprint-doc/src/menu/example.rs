use crate::{DocMenu, Logo};
use yew::prelude::*;
use yewprint::{Icon, Menu, MenuDivider, MenuItem};

pub struct Example {}

pub enum Msg {
    GoToMenu(DocMenu),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let custom_icon = html! {
            <Logo class={classes!("custom-icon")} />
        };

        let share_icon = html! {
            <Icon icon={Icon::Share} />
        };

        html! {
            <>
                <Menu>
                    <MenuItem
                        text={html!("Custom SVG icon")}
                        icon_html={custom_icon}
                    />
                    <MenuDivider />
                    <MenuItem
                        icon={Icon::NewTextBox}
                        text={html!("New text box")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::NewObject}
                        text={html!("New object")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::NewLink}
                        text={html!("New link")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuDivider />
                    <MenuItem
                        icon={Icon::Cog}
                        text={html!("Settings")}
                        label={share_icon}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                </Menu>
                <Menu>
                    <MenuDivider title={html!("Edit")} />
                    <MenuItem
                        icon={Icon::Cut}
                        text={html!("Cut")}
                        label={html!("Ctrl+X")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::Duplicate}
                        text={html!("Copy")}
                        label={html!("Ctrl+C")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::Clipboard}
                        text={html!("Paste")}
                        label={html!("Ctrl+V")}
                        disabled=true
                    />
                    <MenuDivider title={html!("Text")} />
                    <MenuItem
                        icon={Icon::AlignLeft}
                        text={html!("Alignment")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::Style}
                        text={html!("Style")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={Icon::Asterisk}
                        text={html!("Miscellaneous")}
                        href={"#menu"}
                        onclick={ctx.link().callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                </Menu>
            </>
        }
    }
}
