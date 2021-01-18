use crate::{Icon, IconName, Intent, H6};
use boolinator::Boolinator;
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Menu { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ul
                class=classes!(
                    "bp3-menu",
                    self.props.large.as_some("bp3-large"),
                    self.props.class.clone(),
                )
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
    pub href: Option<String>,
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Classes,
    // TODO: pub multiline: bool, (requires <Text>)
    // TODO: popover_props, should_dismiss_popover
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub icon: IconName,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    // TODO: pub children: html::Children,
}

impl Component for MenuItem {
    type Message = ();
    type Properties = MenuItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MenuItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <li>
                <a
                    class=classes!(
                        "bp3-menu-item",
                        self.props.active.as_some("bp3-active"),
                        self.props.disabled.as_some("bp3-disabled"),
                        self.props.intent
                            .or_else(|| self.props.active.as_some(Intent::Primary)),
                        self.props.class.clone(),
                    )
                    href?={(!self.props.disabled).and_option(self.props.href.clone())}
                    tabIndex?={(!self.props.disabled).as_some(0)}
                    onclick={self.props.onclick.clone()}
                >
                    <Icon icon={self.props.icon} />
                    <div class=classes!("bp3-text", "bp3-fill", self.props.text_class.clone())>
                        {self.props.text.clone()}
                    </div>
                    {
                        if let Some(label) = self.props.label.clone() {
                            html! {
                                <span
                                    class=classes!(
                                        "bp3-menu-item-label",
                                        self.props.label_class.clone())
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            if let Some(title) = self.props.title.clone() {
                html! {
                    <li
                        class=classes!("bp3-menu-header")
                    >
                        <H6>{title}</H6>
                    </li>
                }
            } else {
                html! {
                    <li class=classes!("bp3-menu-divider") />
                }
            }
        }
    }
}
