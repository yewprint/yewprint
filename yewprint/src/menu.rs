use crate::icon::{Icon, IconName};
use crate::{ConditionalClass, Intent};
use yew::prelude::*;

pub struct Menu {
    props: MenuProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub class: Option<String>,
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
                class=(
                    "bp3-menu",
                    self.props.large.map_some("bp3-large"),
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
    pub text_class: Option<String>,
    #[prop_or_default]
    pub active: ConditionalClass,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub disabled: ConditionalClass,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Option<String>,
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
                    class=(
                        "bp3-menu-item",
                        self.props.active.map_some("bp3-active"),
                        self.props.disabled.map_some("bp3-disabled"),
                        self.props.intent
                            .or_else(|| self.props.active.map_some(Intent::Primary)),
                        self.props.class.clone(),
                    )
                    href?={(!self.props.disabled).and(self.props.href.clone())}
                    tabIndex?={(!self.props.disabled).map_some(0)}
                    onclick={self.props.onclick.clone()}
                >
                    <Icon icon={self.props.icon} />
                    <div class=("bp3-text", "bp3-fill", self.props.text_class.clone())>
                        {self.props.text.clone()}
                    </div>
                    {
                        if let Some(label) = self.props.label.clone() {
                            html! {
                                <span
                                    class=(
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
