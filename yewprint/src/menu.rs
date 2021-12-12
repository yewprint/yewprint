use crate::{Icon, IconName, Intent, H6};
use std::borrow::Cow;
use yew::prelude::*;

pub struct Menu {
    props: MenuProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub r#ref: NodeRef,
    pub children: html::Children,
}

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <ul
                class={classes!(
                    "bp3-menu",
                    self.props.large.then(|| "bp3-large"),
                    self.props.class.clone(),
                )}
                ref={self.props.r#ref.clone()}
            >
                {self.props.children.clone()}
            </ul>
        }
    }
}

pub struct MenuItem {
    props: MenuItemProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuItemProps {
    #[prop_or_default]
    pub text: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub text_class: Classes,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub href: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Classes,
    // TODO: pub multiline: bool, (requires <Text>)
    // TODO: popover_props, should_dismiss_popover
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub icon_html: Option<Html>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    // TODO: pub children: html::Children,
}

impl Component for MenuItem {
    type Message = ();
    type Properties = MenuItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <li>
                <a
                    class={classes!(
                        "bp3-menu-item",
                        self.props.active.then(|| "bp3-active"),
                        self.props.disabled.then(|| "bp3-disabled"),
                        self.props.intent
                            .or_else(|| self.props.active.then(|| Intent::Primary)),
                        self.props.class.clone(),
                    )}
                    href={(!self.props.disabled).then(|| self.props.href.clone()).flatten()}
                    tabIndex={(!self.props.disabled).then(|| "0")}
                    onclick={(!self.props.disabled).then(|| self.props.onclick.clone())}
                >
                    {
                        if let Some(icon_name) = self.props.icon {
                            html! {
                                <Icon icon={icon_name} />
                            }
                        } else if let Some(html) = self.props.icon_html.clone() {
                            html
                        } else {
                            html! {
                                <Icon icon={IconName::Blank} />
                            }
                        }
                    }
                    <div class={classes!("bp3-text", "bp3-fill", self.props.text_class.clone())}>
                        {self.props.text.clone()}
                    </div>
                    {
                        if let Some(label) = self.props.label.clone() {
                            html! {
                                <span
                                    class={classes!(
                                        "bp3-menu-item-label",
                                        self.props.label_class.clone())}
                                >
                                    {label}
                                </span>
                            }
                        } else {
                            html!()
                        }
                    }

                </a>
            </li>
        }
    }
}

pub struct MenuDivider {
    props: MenuDividerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuDividerProps {
    #[prop_or_default]
    pub title: Option<yew::virtual_dom::VNode>,
}

impl Component for MenuDivider {
    type Message = ();
    type Properties = MenuDividerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            {if let Some(title) = self.props.title.clone() {
                html! {
                    <li
                        class={classes!("bp3-menu-header")}
                    >
                        <H6>{title}</H6>
                    </li>
                }
            } else {
                html! {
                    <li class={classes!("bp3-menu-divider")} />
                }
            }}
        }
    }
}
