use crate::{DocMenu, Logo};
use yew::prelude::*;
use yewprint::{Icon, IconName, Menu, MenuDivider, MenuItem};

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
            <Icon icon={IconName::Share} />
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
                        icon={IconName::NewTextBox}
                        text={html!("New text box")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::NewObject}
                        text={html!("New object")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::NewLink}
                        text={html!("New link")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuDivider />
                    <MenuItem
                        icon={IconName::Cog}
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
                        icon={IconName::Cut}
                        text={html!("Cut")}
                        label={html!("Ctrl+X")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::Duplicate}
                        text={html!("Copy")}
                        label={html!("Ctrl+C")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::Clipboard}
                        text={{html!("Paste")}}
                        label={{html!("Ctrl+V")}}
                        disabled=true
                    />
                    <MenuDivider title={{html!("Text")}} />
                    <MenuItem
                        icon={IconName::AlignLeft}
                        text={html!("Alignment")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::Style}
                        text={html!("Style")}
                        href={"#menu"}
                        onclick={ctx.link()
                            .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                    <MenuItem
                        icon={IconName::Asterisk}
                        text={html!("Miscellaneous")}
                        href={"#menu"}
                        onclick={ctx.link().callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                    />
                </Menu>
            </>
        }
    }
}
