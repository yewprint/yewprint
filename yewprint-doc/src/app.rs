use crate::buttons::*;
use crate::callout::*;
use crate::card::*;
use crate::collapse::*;
use crate::icon::*;
use crate::progressbar::*;
use crate::text::*;
use crate::tree::*;

use yew::prelude::*;
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    router::Router,
    Switch,
};
use yewprint::{ConditionalClass, IconName, Menu, MenuItem};

pub struct App {
    link: ComponentLink<Self>,
    dark_theme: ConditionalClass,
    route_dispatcher: RouteAgentDispatcher,
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
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight => *self.dark_theme ^= true,
            Msg::GoToMenu(doc_menu) => {
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(doc_menu.into()));
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
                                    text={html!("Card")}
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Card))
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
                                    text={html!("ProgressBar")}
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::ProgressBar))
                                />
                                <MenuItem
                                    text={html!("Switch")}
                                    onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Switch))
                                />
                                <MenuItem
                                    text={html!("Text")}
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Text))
                                />
                                <MenuItem
                                    text={html!("Tree")}
                                    onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Tree))
                                />
                                // NOTE: thanks to keep this list of <MenuItem> sorted
                                //       alphabetically (except for the light switch)
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
                            <Router<DocMenu, ()>
                                render=Router::render(|switch: DocMenu| {
                                    match switch {
                                        DocMenu::Button | DocMenu::Home => html! (<ButtonDoc />),
                                        DocMenu::Switch => html! (),
                                        DocMenu::Callout => html!(<CalloutDoc />),
                                        DocMenu::Card => html!(<CardDoc />),
                                        DocMenu::Collapse => html!(<CollapseDoc />),
                                        DocMenu::Text => html!(<TextDoc />),
                                        DocMenu::Tree => html!(<TreeDoc />),
                                        DocMenu::Icon => html!(<IconDoc />),
                                        DocMenu::ProgressBar => html!(<ProgressBarDoc />),
                                        DocMenu::Menu => html!(),
                                    }
                                })
                            />
                        </div>
                    </main>
                </div>
            </div>
        }
    }
}

#[derive(Debug, Copy, Clone, Switch)]
pub enum DocMenu {
    #[to = "/#button"]
    Button,
    #[to = "/#callout"]
    Callout,
    #[to = "/#card"]
    Card,
    #[to = "/#collapse"]
    Collapse,
    #[to = "/#icon"]
    Icon,
    #[to = "/#menu"]
    Menu,
    #[to = "/#progress-bar"]
    ProgressBar,
    #[to = "/#switch"]
    Switch,
    #[to = "/#text"]
    Text,
    #[to = "/#tree"]
    Tree,
    #[to = "/"]
    Home,
}
