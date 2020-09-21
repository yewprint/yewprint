use crate::buttons::doc::*;
use crate::collapse::doc::*;
use crate::forms::controls::doc::SwitchDoc;
use crate::icon::doc::*;
use crate::menu::*;
use crate::tree::doc::*;
use yew::prelude::*;

const DARK_BG_COLOR: &str = "#30404d";
const LIGHT_BG_COLOR: &str = "#f5f8fa";
const DARK_FG_COLOR: &str = "#f5f8fa";
const LIGHT_FG_COLOR: &str = "#182026";

pub struct App {
    link: ComponentLink<Self>,
    doc_menu: DocMenu,
    dark_theme: bool,
}

pub enum Msg {
    ToggleLight,
    GoToMenu(DocMenu),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            dark_theme: true,
            doc_menu: DocMenu::Tree,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight => self.dark_theme ^= true,
            Msg::GoToMenu(doc_menu) => {
                self.doc_menu = doc_menu;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let (fg_color, bg_color) = if self.dark_theme {
            (DARK_FG_COLOR, DARK_BG_COLOR)
        } else {
            (LIGHT_FG_COLOR, LIGHT_BG_COLOR)
        };
        let style = format!(
            "min-height: 100vh; padding: 10px; background-color: {}; color: {}",
            bg_color, fg_color
        );
        let class = if self.dark_theme { "bp3-dark" } else { "" };

        html! {
            <div class={class} style={style}>
                <div>
                    <Menu>
                        <MenuItem text={html!("Button")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Button)) />
                        <MenuItem text={html!("Collapse")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Collapse)) />
                        <MenuItem text={html!("Icon")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Icon)) />
                        <MenuItem text={html!("Menu")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Menu)) />
                        <MenuItem text={html!("Switch")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Switch)) />
                        <MenuItem text={html!("Tree")} onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Tree)) />
                    </Menu>
                </div>
                {
                    match self.doc_menu {
                        DocMenu::Button => html! (<ButtonDoc />),
                        DocMenu::Switch => html! (<SwitchDoc
                            dark_theme=self.dark_theme
                            onclick=self.link.callback(|_| Msg::ToggleLight)
                            />),
                        DocMenu::Collapse => html!(<CollapseDoc />),
                        DocMenu::Tree => html!(<TreeDoc />),
                        DocMenu::Icon => html!(<IconDoc />),
                        DocMenu::Menu => html!(),
                    }
                }
            </div>
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DocMenu {
    Button,
    Collapse,
    Icon,
    Menu,
    Switch,
    Tree,
}
