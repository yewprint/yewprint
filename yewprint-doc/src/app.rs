use crate::button_group::*;
use crate::buttons::*;
use crate::callout::*;
use crate::card::*;
use crate::checkbox::*;
use crate::collapse::*;
use crate::control_group::*;
use crate::divider::*;
use crate::html_select::*;
use crate::icon::*;
use crate::input_group::*;
use crate::menu::*;
use crate::numeric_input::*;
use crate::panel_stack::*;
use crate::progress_bar::*;
use crate::radio::*;
use crate::slider::*;
use crate::spinner::*;
use crate::switch::*;
use crate::tabs::*;
use crate::tag::*;
use crate::text::*;
use crate::text_area::*;
use crate::tree::*;
use std::borrow::Cow;
use yew::prelude::*;
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    router::Router,
    Switch,
};
use yewprint::{IconName, Menu, MenuItem};

pub struct App {
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

    fn create(ctx: &Context<Self>) -> Self {
        App {
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true),
            route_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleLight => self.dark_theme ^= true,
            Msg::GoToMenu(doc_menu) => {
                self.route_dispatcher
                    .send(RouteRequest::ChangeRoute(doc_menu.into()));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let menu = html! {
            <Menu>
                <MenuItem
                    text={html!(go_to_theme_label)}
                    onclick={ctx.link()
                        .callback(|_| Msg::ToggleLight)}
                    icon={go_to_theme_icon}
                />
                <MenuItem
                    text={html!("Button")}
                    href={"#button"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Button))}
                />
                <MenuItem
                    text={html!("ButtonGroup")}
                    href={"#button-group"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::ButtonGroup))}
                />
                <MenuItem
                    text={html!("Callout")}
                    href={"#callout"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Callout))}
                />
                <MenuItem
                    text={html!("Card")}
                    href={"#card"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Card))}
                />
                <MenuItem
                    text={html!("Checkbox")}
                    href={"#checkbox"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Checkbox))}
                />
                <MenuItem
                    text={html!("Collapse")}
                    href={"#collapse"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Collapse))}
                />
                <MenuItem
                    text={html!("ControlGroup")}
                    href={"#control-group"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::ControlGroup))}
                    />
                <MenuItem
                    text={html!("Divider")}
                    href={"#divider"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Divider))}
                />
                <MenuItem
                    text={html!("HtmlSelect")}
                    href={"#html-select"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::HtmlSelect))}
                />
                <MenuItem
                    text={html!("Icon")}
                    href={"#icon"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Icon))}
                />
                <MenuItem
                    text={html!("InputGroup")}
                    href={"#input-group"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::InputGroup))}
                />
                <MenuItem
                    text={html!("Menu")}
                    href={"#menu"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Menu))}
                />
                <MenuItem
                    text={html!("NumericInput")}
                    href={"#numeric-input"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::NumericInput))}
                />
                <MenuItem
                    text={html!("PanelStack")}
                    href={"#panel-stack"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::PanelStack))}
                />
                <MenuItem
                    text={html!("ProgressBar")}
                    href={"#progress-bar"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::ProgressBar))}
                />
                <MenuItem
                    text={html!("Radio")}
                    href={"#radio"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Radio))}
                />
                <MenuItem
                    text={html!("Slider")}
                    href={"#slider"}
                    onclick={ctx.link().callback(|_| Msg::GoToMenu(DocMenu::Slider))}
                />
                <MenuItem
                    text={html!("Spinner")}
                    href={"#spinner"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Spinner))}
                />
                <MenuItem
                    text={html!("Switch")}
                    href={"#switch"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Switch))}
                />
                <MenuItem
                    text={html!("Tabs")}
                    href={"#tabs"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Tabs))}
                />
                <MenuItem
                    text={html!("Tag")}
                    href={"#tag"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Tag))}
                />
                <MenuItem
                    text={html!("Text")}
                    href={"#text"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Text))}
                />
                <MenuItem
                    text={html!("TextArea")}
                    href={"#textarea"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::TextArea))}
                />
                <MenuItem
                    text={html!("Tree")}
                    href={"#tree"}
                    onclick={ctx.link()
                        .callback(|_| Msg::GoToMenu(DocMenu::Tree))}
                />
                // NOTE: thanks to keep this list of <MenuItem> sorted
                //       alphabetically (except for the light switch)
            </Menu>
        };

        let navigation = html! {
            <div class={classes!("docs-nav-wrapper")}>
                <div class={classes!("docs-nav")}>
                    <div class={classes!("docs-nav-title")}>
                        <a class={classes!("docs-logo")} href="/">
                            {crate::include_raw_html!("logo.svg")}
                        </a>
                        <div>
                            <div class={classes!("bp3-navbar-heading", "docs-heading")}>
                                {"Yewprint"}
                            </div>
                            <a
                                class={classes!("bp3-text-muted")}
                                href="https://github.com/yewprint/yewprint"
                                target="_blank"
                            >
                                <small>{"View on GitHub"}</small>
                            </a>
                        </div>
                    </div>
                    {{ menu }}
                    <div class="docs-nav-sponsors">
                        <a href={"https://www.netlify.com"}>
                            <img
                                src={netlify_badge}
                                alt="Deploys by Netlify"
                            />
                        </a>
                    </div>
                </div>
            </div>
        };

        html! {
            <div class={classes!("docs-root", self.dark_theme.then(|| "bp3-dark"))}>
                <div class={classes!("docs-app")}>
                    {{ navigation }}
                    <main class={classes!("docs-content-wrapper")} role="main">
                        <div class={classes!("docs-page")}>
                            <Router<DocMenu, ()>
                                render={Router::render(|switch: DocMenu| {
                                    match switch {
                                        DocMenu::Button | DocMenu::Home => html! (<ButtonDoc />),
                                        DocMenu::ButtonGroup => html! (<ButtonGroupDoc />),
                                        DocMenu::Callout => html!(<CalloutDoc />),
                                        DocMenu::Card => html!(<CardDoc />),
                                        DocMenu::Checkbox => html!(<CheckboxDoc />),
                                        DocMenu::Collapse => html!(<CollapseDoc />),
                                        DocMenu::ControlGroup => html!(<ControlGroupDoc />),
                                        DocMenu::Divider => html!(<DividerDoc />),
                                        DocMenu::HtmlSelect => html!(<HtmlSelectDoc />),
                                        DocMenu::Icon => html!(<IconDoc />),
                                        DocMenu::InputGroup => html!(<InputGroupDoc />),
                                        DocMenu::Menu => html!(<MenuDoc />),
                                        DocMenu::NumericInput => html!(<NumericInputDoc />),
                                        DocMenu::PanelStack => html!(<PanelStackDoc />),
                                        DocMenu::ProgressBar => html!(<ProgressBarDoc />),
                                        DocMenu::Radio => html!(<RadioDoc />),
                                        DocMenu::Slider => html!(<SliderDoc />),
                                        DocMenu::Spinner => html!(<SpinnerDoc />),
                                        DocMenu::Switch => html!(<SwitchDoc />),
                                        DocMenu::Tabs => html!(<TabsDoc />),
                                        DocMenu::Tag => html!(<TagDoc />),
                                        DocMenu::Text => html!(<TextDoc />),
                                        DocMenu::TextArea => html!(<TextAreaDoc />),
                                        DocMenu::Tree => html!(<TreeDoc />),
                                    }
                                })}
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
    #[to = "/#checkbox"]
    Checkbox,
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
    #[to = "/#numeric-input"]
    NumericInput,
    #[to = "/#panel-stack"]
    PanelStack,
    #[to = "/#progress-bar"]
    ProgressBar,
    #[to = "/#radio"]
    Radio,
    #[to = "/#slider"]
    Slider,
    #[to = "/#spinner"]
    Spinner,
    #[to = "/#switch"]
    Switch,
    #[to = "/#tabs"]
    Tabs,
    #[to = "/#tag"]
    Tag,
    #[to = "/#textarea"]
    TextArea,
    #[to = "/#text"]
    Text,
    #[to = "/#tree"]
    Tree,
    #[to = "/"]
    Home,
}
