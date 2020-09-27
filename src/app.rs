use crate::buttons::doc::*;
use crate::collapse::doc::*;
use crate::icon::doc::*;
use crate::switch::doc::*;
use crate::tree::doc::*;
use crate::{ConditionalClass, Menu, MenuItem};
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    doc_menu: DocMenu,
    dark_theme: ConditionalClass,
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
            dark_theme: true.into(),
            doc_menu: DocMenu::Button,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight => *self.dark_theme ^= true,
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
        html! {
            <div class=("docs-app", self.dark_theme.map_some("bp3-dark"))>
                <div class="docs-nav-wrapper">
                    <div class="docs-nav">
                        <div class="docs-nav-title">
                            <a class="docs-logo" href="/">
                                {crate::include_raw_html!("logo.svg")}
                            </a>
                            <div>
                                <div class="docs-heading">
                                    {"Yewprint"}
                                </div>
                                <a
                                    class="bp3-text-muted"
                                    href="https://github.com/cecton/yewprint"
                                    target="_blank"
                                >
                                    <small>{"View on GitHub"}</small>
                                </a>
                            </div>
                        </div>
                        <Menu>
                            <MenuItem
                                text={html!("Button")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Button))
                            />
                            <MenuItem
                                text={html!("Collapse")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Collapse))
                            />
                            <MenuItem
                                text={html!("Icon")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Icon))
                            />
                            <MenuItem
                                text={html!("Menu")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Menu))
                            />
                            <MenuItem
                                text={html!("Switch")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Switch))
                            />
                            <MenuItem
                                text={html!("Tree")}
                                onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Tree))
                            />
                        </Menu>
                        <div class="docs-nav-sponsors">
                            <a href="https://www.netlify.com">
                                <img
                                    src="https://www.netlify.com/img/global/badges/netlify-color-accent.svg"
                                    alt="Deploys by Netlify"
                                />
                            </a>
                        </div>
                    </div>
                </div>
                <main class="docs-content-wrapper" role="main">
                    <div class="docs-page">
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
                </main>
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
