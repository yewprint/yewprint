use crate::{Icon, IconName};
use yew::prelude::*;

pub struct Breadcrumbs {
    props: BreadcrumbsProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BreadcrumbsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub collapse_from: Boundary,
    #[prop_or_default]
    pub min_visible_itens: usize,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Clone, PartialEq)]
pub enum Boundary {
    Start,
    End,
}

impl Default for Boundary {
    fn default() -> Self {
        Boundary::End
    }
}

impl Component for Breadcrumbs {
    type Message = ();

    type Properties = BreadcrumbsProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Breadcrumbs { props }
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
                    "bp3-breadcrumbs",
                    self.props.class.clone()

            )>
            {
                self.props.children.clone()
            }
            </ul>
        }
    }
}
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct BreadcrumbItemProps {
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub disabled: bool,
    pub text: String,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub target: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub current: bool,
    #[prop_or_default]
    pub intent: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    props: BreadcrumbItemProps,
    link: ComponentLink<Self>,
}

impl PartialEq for BreadcrumbItem {
    fn eq(&self, other: &Self) -> bool {
        self.props.eq(&other.props)
    }
}

impl Component for BreadcrumbItem {
    type Properties = BreadcrumbItemProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let classes = classes!(
            "bp3-breadcrumb",
            self.props.current.then(|| "bp3-breadcrumb-current"),
            self.props.disabled.then(|| "bp3-disabled"),
            self.props.class.clone()
        );
        let inner = html! {
            <>
            {
                if let Some(icon) = self.props.icon {
                    html! {
                        <Icon icon=icon />
                    }
                } else {
                    html!()
                }
            }

            {
                self.props.text.clone()

            }
            {
                self.props.children.clone()
            }
            </>
        };
        if self.props.href.is_some() || self.props.onclick.is_some() {
            html! {
                <li>
                    <a
                        class=classes
                        href?=self.props.href.clone()
                        onclick?=self.props.onclick.clone()
                    >
                    {
                        inner
                    }
                    </a>
                </li>
            }
        } else {
            html! {
                <li>
                    <span
                        class=classes
                    >
                    {
                        inner
                    }
                    </span>
                </li>
            }
        }
    }
}
