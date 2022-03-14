use crate::{Icon, IconName, Intent, H6};
use yew::prelude::*;

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

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <ul
            class={classes!(
                "bp3-menu",
                props.large.then(|| "bp3-large"),
                props.class.clone(),
            )}
            ref={props.r#ref.clone()}
        >
            {props.children.clone()}
        </ul>
    }
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
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub icon_html: Option<Html>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    // TODO: pub children: html::Children,
}

#[function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> Html {
    html! {
        <li>
            <a
                class={classes!(
                    "bp3-menu-item",
                    props.active.then(|| "bp3-active"),
                    props.disabled.then(|| "bp3-disabled"),
                    props.intent
                        .or_else(|| props.active.then(|| Intent::Primary)),
                    props.class.clone(),
                )}
                href={(!props.disabled).then(|| props.href.clone()).flatten()}
                tabIndex={(!props.disabled).then(|| "0")}
                onclick={(!props.disabled).then(|| props.onclick.clone())}
            >
                {
                    if let Some(icon_name) = props.icon {
                        html! {
                            <Icon icon={icon_name} />
                        }
                    } else if let Some(html) = props.icon_html.clone() {
                        html
                    } else {
                        html! {
                            <Icon icon={IconName::Blank} />
                        }
                    }
                }
                <div class={classes!("bp3-text", "bp3-fill", props.text_class.clone())}>
                    {props.text.clone()}
                </div>
                {
                    if let Some(label) = props.label.clone() {
                        html! {
                            <span
                                class={classes!(
                                    "bp3-menu-item-label",
                                    props.label_class.clone())}
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

#[derive(Clone, PartialEq, Properties)]
pub struct MenuDividerProps {
    #[prop_or_default]
    pub title: Option<yew::virtual_dom::VNode>,
}

#[function_component(MenuDivider)]
fn menu_divider(props: &MenuDividerProps) -> Html {
    html! {
        {if let Some(title) = props.title.clone() {
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
