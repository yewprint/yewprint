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
use yew::prelude::*;
use yew_router::prelude::*;
use yewprint::{IconName, Menu, MenuItem};

#[function_component(AppRoot)]
pub fn app_root() -> Html {
    html! {
        <BrowserRouter>
            <App></App>
        </BrowserRouter>
    }
}

pub struct App {
    dark_theme: bool,
}

pub enum Msg {
    ToggleLight,
    GoToMenu(MouseEvent, DocMenu),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            dark_theme: web_sys::window()
                .and_then(|x| x.match_media("(prefers-color-scheme: dark)").ok().flatten())
                .map(|x| x.matches())
                .unwrap_or(true),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleLight => self.dark_theme ^= true,
            Msg::GoToMenu(event, doc_menu) => {
                event.prevent_default();
                if let Some(navigator) = ctx.link().navigator() {
                    navigator.push(&doc_menu);
                } else {
                    gloo::console::warn!("Could not get history from Context");
                }
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
                    href="/button"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Button))}
                />
                <MenuItem
                    text={html!("ButtonGroup")}
                    href="/button-group"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::ButtonGroup))}
                />
                <MenuItem
                    text={html!("Callout")}
                    href="/callout"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Callout))}
                />
                <MenuItem
                    text={html!("Card")}
                    href="/card"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Card))}
                />
                <MenuItem
                    text={html!("Checkbox")}
                    href="/progress-bar"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Checkbox))}
                />
                <MenuItem
                    text={html!("Collapse")}
                    href="/collapse"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Collapse))}
                />
                <MenuItem
                    text={html!("ControlGroup")}
                    href="/control-group"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::ControlGroup))}
                    />
                <MenuItem
                    text={html!("Divider")}
                    href="/divider"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Divider))}
                />
                <MenuItem
                    text={html!("HtmlSelect")}
                    href="/html-select"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::HtmlSelect))}
                />
                <MenuItem
                    text={html!("Icon")}
                    href="/icon"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Icon))}
                />
                <MenuItem
                    text={html!("InputGroup")}
                    href="/input-group"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::InputGroup))}
                />
                <MenuItem
                    text={html!("Menu")}
                    href="/menu"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Menu))}
                />
                <MenuItem
                    text={html!("NumericInput")}
                    href="/numeric-input"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::NumericInput))}
                />
                <MenuItem
                    text={html!("PanelStack")}
                    href="/panel-stack"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::PanelStack))}
                />
                <MenuItem
                    text={html!("ProgressBar")}
                    href="/progress-bar"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::ProgressBar))}
                />
                <MenuItem
                    text={html!("Radio")}
                    href="/radio"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Radio))}
                />
                <MenuItem
                    text={html!("Slider")}
                    href="/slider"
                    onclick={ctx.link().callback(|e| Msg::GoToMenu(e, DocMenu::Slider))}
                />
                <MenuItem
                    text={html!("Spinner")}
                    href="/spinner"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Spinner))}
                />
                <MenuItem
                    text={html!("Switch")}
                    href="/switch"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Switch))}
                />
                <MenuItem
                    text={html!("Tabs")}
                    href="/tabs"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Tabs))}
                />
                <MenuItem
                    text={html!("Tag")}
                    href="/tag"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Tag))}
                />
                <MenuItem
                    text={html!("Text")}
                    href="/text"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Text))}
                />
                <MenuItem
                    text={html!("TextArea")}
                    href="/text-area"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::TextArea))}
                />
                <MenuItem
                    text={html!("Tree")}
                    href="/tree"
                    onclick={ctx.link()
                        .callback(|e| Msg::GoToMenu(e, DocMenu::Tree))}
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
            <div class={classes!("docs-root", self.dark_theme.then_some("bp3-dark"))}>
                <div class={classes!("docs-app")}>
                    {{ navigation }}
                    <main class={classes!("docs-content-wrapper")} role="main">
                        <div class={classes!("docs-page")}>
                                <Switch<DocMenu> render={switch} />
                        </div>
                    </main>
                </div>
            </div>
        }
    }
}

fn switch(route: DocMenu) -> Html {
    match route {
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
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Routable)]
pub enum DocMenu {
    #[at("/button-group")]
    ButtonGroup,
    #[at("/button")]
    Button,
    #[at("/callout")]
    Callout,
    #[at("/card")]
    Card,
    #[at("/checkbox")]
    Checkbox,
    #[at("/collapse")]
    Collapse,
    #[at("/control-group")]
    ControlGroup,
    #[at("/html-select")]
    HtmlSelect,
    #[at("/divider")]
    Divider,
    #[at("/icon")]
    Icon,
    #[at("/input-group")]
    InputGroup,
    #[at("/menu")]
    Menu,
    #[at("/numeric-input")]
    NumericInput,
    #[at("/panel-stack")]
    PanelStack,
    #[at("/progress-bar")]
    ProgressBar,
    #[at("/radio")]
    Radio,
    #[at("/slider")]
    Slider,
    #[at("/spinner")]
    Spinner,
    #[at("/switch")]
    Switch,
    #[at("/tabs")]
    Tabs,
    #[at("/tag")]
    Tag,
    #[at("/text-area")]
    TextArea,
    #[at("/text")]
    Text,
    #[at("/tree")]
    Tree,
    #[not_found]
    #[at("/")]
    Home,
}
