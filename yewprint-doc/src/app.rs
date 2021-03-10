use crate::button_group::*;
use crate::buttons::*;
use crate::callout::*;
use crate::card::*;
use crate::collapse::*;
use crate::control_group::*;
use crate::divider::*;
use crate::html_select::*;
use crate::icon::*;
use crate::input_group::*;
use crate::menu::*;
use crate::progressbar::*;
use crate::slider::*;
use crate::switch::*;
use crate::tabs::*;
use crate::tag::*;
use crate::text::*;
use crate::tree::*;
use boolinator::Boolinator;
use yew::prelude::*;
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    router::Router,
    Switch,
};
use yewprint::{IconName, Menu, MenuItem};

pub struct App {
    link: ComponentLink<Self>,
    dark_theme: bool,
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
                .unwrap_or(true),
            link,
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight => self.dark_theme ^= true,
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
        let netlify_badge = if self.dark_theme {
            "https://www.netlify.com/img/global/badges/netlify-color-accent.svg"
        } else {
            "https://www.netlify.com/img/global/badges/netlify-color-bg.svg"
        };
        let go_to_theme_label = if self.dark_theme {
            "Light theme"
        } else {
            "Dark theme"
        };
        let go_to_theme_icon = if self.dark_theme {
            IconName::Flash
        } else {
            IconName::Moon
        };

        html! {
            <div class=classes!("docs-root", self.dark_theme.as_some("bp3-dark"))>
                <div class=classes!("docs-app")>
                    <div class=classes!("docs-nav-wrapper")>
                        <div class=classes!("docs-nav")>
                            <div class=classes!("docs-nav-title")>
                                <a class=classes!("docs-logo") href="/">
                                    {crate::include_raw_html!("logo.svg")}
                                </a>
                                <div>
                                    <div class=classes!("bp3-navbar-heading", "docs-heading")>
                                        {"Yewprint"}
                                    </div>
                                    <a
                                        class=classes!("bp3-text-muted")
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
                                    onclick=self.link
                                        .callback(|_| Msg::ToggleLight)
                                    icon=go_to_theme_icon
                                />
                                <MenuItem
                                    text={html!("Button")}
                                    href="#button"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Button))
                                />
                                <MenuItem
                                    text={html!("ButtonGroup")}
                                    href="#button-group"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::ButtonGroup))
                                />
                                <MenuItem
                                    text={html!("Callout")}
                                    href="#callout"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Callout))
                                />
                                <MenuItem
                                    text={html!("Card")}
                                    href="#card"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Card))
                                />
                                <MenuItem
                                    text={html!("Collapse")}
                                    href="#collapse"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Collapse))
                                />
                                <MenuItem
                                    text={html!("ControlGroup")}
                                    href="#control-group"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::ControlGroup))
                                    />
                                <MenuItem
                                    text={html!("Divider")}
                                    href="#divider"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Divider))
                                />
                                <MenuItem
                                    text={html!("HtmlSelect")}
                                    href="#html-select"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::HtmlSelect))
                                />
                                <MenuItem
                                    text={html!("Icon")}
                                    href="#icon"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Icon))
                                />
                                <MenuItem
                                    text={html!("InputGroup")}
                                    href="#input-group"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::InputGroup))
                                />
                                <MenuItem
                                    text={html!("Menu")}
                                    href="#menu"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Menu))
                                />
                                <MenuItem
                                    text={html!("ProgressBar")}
                                    href="#progress-bar"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::ProgressBar))
                                />
                                <MenuItem
                                    text={html!("Slider")}
                                    href="#slider"
                                    onclick=self.link.callback(|_| Msg::GoToMenu(DocMenu::Slider))
                                />
                                <MenuItem
                                    text={html!("Switch")}
                                    href="#switch"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Switch))
                                />
                                <MenuItem
                                    text={html!("Tabs")}
                                    href="#tabs"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Tabs))
                                />
                                <MenuItem
                                    text={html!("Tag")}
                                    href="#tag"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Tag))
                                />
                                <MenuItem
                                    text={html!("Text")}
                                    href="#text"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Text))
                                />
                                <MenuItem
                                    text={html!("Tree")}
                                    href="#tree"
                                    onclick=self.link
                                        .callback(|_| Msg::GoToMenu(DocMenu::Tree))
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
                    <main class=classes!("docs-content-wrapper") role="main">
                        <div class=classes!("docs-page")>
                            <Router<DocMenu, ()>
                                render=Router::render(|switch: DocMenu| {
                                    match switch {
                                        DocMenu::Button | DocMenu::Home => html! (<ButtonDoc />),
                                        DocMenu::ButtonGroup => html! (<ButtonGroupDoc />),
                                        DocMenu::Callout => html!(<CalloutDoc />),
                                        DocMenu::Card => html!(<CardDoc />),
                                        DocMenu::Collapse => html!(<CollapseDoc />),
                                        DocMenu::ControlGroup => html!(<ControlGroupDoc />),
                                        DocMenu::Divider => html!(<DividerDoc />),
                                        DocMenu::HtmlSelect => html!(<HtmlSelectDoc />),
                                        DocMenu::Icon => html!(<IconDoc />),
                                        DocMenu::InputGroup => html!(<InputGroupDoc />),
                                        DocMenu::Menu => html!(<MenuDoc />),
                                        DocMenu::ProgressBar => html!(<ProgressBarDoc />),
                                        DocMenu::Slider => html!(<SliderDoc />),
                                        DocMenu::Switch => html!(<SwitchDoc />),
                                        DocMenu::Tabs => html!(<TabsDoc />),
                                        DocMenu::Tag => html!(<TagDoc />),
                                        DocMenu::Text => html!(<TextDoc />),
                                        DocMenu::Tree => html!(<TreeDoc />),
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
    #[to = "/#button-group"]
    ButtonGroup,
    #[to = "/#button"]
    Button,
    #[to = "/#callout"]
    Callout,
    #[to = "/#card"]
    Card,
    #[to = "/#collapse"]
    Collapse,
    #[to = "/#control-group"]
    ControlGroup,
    #[to = "/#html-select"]
    HtmlSelect,
    #[to = "/#divider"]
    Divider,
    #[to = "/#icon"]
    Icon,
    #[to = "/#input-group"]
    InputGroup,
    #[to = "/#menu"]
    Menu,
    #[to = "/#progress-bar"]
    ProgressBar,
    #[to = "/#slider"]
    Slider,
    #[to = "/#switch"]
    Switch,
    #[to = "/#tabs"]
    Tabs,
    #[to = "/#tag"]
    Tag,
    #[to = "/#text"]
    Text,
    #[to = "/#tree"]
    Tree,
    #[to = "/"]
    Home,
}
