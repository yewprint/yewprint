use crate::buttons::*;
use crate::callout::*;
use crate::collapse::*;
use crate::icon::*;
use crate::switch::*;
use crate::tree::*;
use crate::progressbar::*;

use yew::prelude::*;
use yewprint::{ConditionalClass, IconName, Menu, MenuItem};

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
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true)
                .into(),
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
        let netlify_badge = if *self.dark_theme {
            "https://www.netlify.com/img/global/badges/netlify-color-accent.svg"
        } else {
            "https://www.netlify.com/img/global/badges/netlify-color-bg.svg"
        };
        let go_to_theme_label = if *self.dark_theme {
            "Light theme"
        } else {
            "Dark theme"
        };
        let go_to_theme_icon = if *self.dark_theme {
            IconName::Flash
        } else {
            IconName::Moon
        };

        html! {
            <div class=("docs-root", self.dark_theme.map_some("bp3-dark"))>
                <div class="docs-app">
                    <div class="docs-nav-wrapper">
                        <div class="docs-nav">
                            <div class="docs-nav-title">
                                <a class="docs-logo" href="/">
                                    {crate::include_raw_html!("logo.svg")}
                                </a>
                                <div>
                                    <div class="bp3-navbar-heading docs-heading">
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
                                    text={html!(go_to_theme_label)}
                                    onclick=self.link.callback(|_| Msg::ToggleLight)
                                    icon=go_to_theme_icon
                                />
                                <MenuItem
                                    text={html!("Button")}
                                    onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Button))
                                />
                                <MenuItem
                                    text={html!("Callout")}
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Callout))
                                />
                                <MenuItem
                                    text={html!("Collapse")}
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Collapse))
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
                                <MenuItem
                                    text={html!("ProgressBar")}
                                    onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::ProgressBar))
                                />
                            </Menu>
                            <div class="docs-nav-sponsors">
                                <a href="https://www.netlify.com">
                                    <img
                                        src=netlify_badge
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
                                    DocMenu::Callout => html!(<CalloutDoc />),
                                    DocMenu::Collapse => html!(<CollapseDoc />),
                                    DocMenu::Tree => html!(<TreeDoc />),
                                    DocMenu::Icon => html!(<IconDoc />),
                                    DocMenu::ProgressBar => html!(<ProgressBarDoc />),
                                    DocMenu::Menu => html!(),
                                }
                            }
                        </div>
                    </main>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DocMenu {
    Button,
    Callout,
    Collapse,
    Icon,
    Menu,
    Switch,
    Tree,
    ProgressBar,
}
