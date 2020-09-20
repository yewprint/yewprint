use crate::icon::{Icon, IconName};
use crate::Intent;
use yew::prelude::*;

pub struct Menu {
    props: MenuProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub large: bool,
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
        let mut class = Classes::from("bp3-menu");
        if self.props.large {
            class.push("bp3-large");
        }
        class = class.extend(self.props.class.clone());

        html! {
            <ul class=class ref={self.props.r#ref.clone()}>
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
    pub active: bool,
    #[prop_or_default]
    pub class: Option<String>,
    // TODO: pub disabled: bool,
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
        let mut anchor_class = Classes::from("bp3-menu-item");
        if self.props.active {
            anchor_class.push("bp3-active");
        }
        if let Some(intent) = self.props.intent.clone() {
            anchor_class = anchor_class.extend(intent);
        } else if self.props.active {
            anchor_class = anchor_class.extend(Intent::Primary);
        }
        anchor_class = anchor_class.extend(self.props.class.clone());

        let mut text_class = Classes::from("bp3-text");
        text_class.push("bp3-fill");
        text_class = text_class.extend(self.props.text_class.clone());

        html! {
            <li>
                <a
                    class=anchor_class
                    onclick={self.props.onclick.clone()}
                >
                    <Icon icon={self.props.icon} />
                    <div class=text_class>
                        {self.props.text.clone()}
                    </div>
                    {
                        if let Some(label) = self.props.label.clone() {
                            let mut label_class = Classes::from("bp3-menu-item-label");
                            label_class = label_class.extend(self.props.label_class.clone());

                            html!(<span class=label_class>{label}</span>)
                        } else {
                            html!()
                        }
                    }
                </a>
            </li>
        }
    }
}
