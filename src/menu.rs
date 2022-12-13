use crate::{Icon, IconName, Intent, H6};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

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
                props.large.then_some("bp3-large"),
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
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub label: Option<Html>,
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
pub fn menu_item(
    MenuItemProps {
        text,
        text_class,
        active,
        class,
        disabled,
        href,
        label,
        label_class,
        intent,
        icon,
        icon_html,
        onclick,
    }: &MenuItemProps,
) -> Html {
    html! {
        <li>
            <a
                class={classes!(
                    "bp3-menu-item",
                    active.then_some("bp3-active"),
                    disabled.then_some("bp3-disabled"),
                    intent
                        .or_else(|| active.then_some(Intent::Primary)),
                    class.clone(),
                )}
                href={(!disabled).then(|| href.clone()).flatten()}
                tabIndex={(!disabled).then_some("0")}
                onclick={(!disabled).then(|| onclick.clone())}
            >
                {
                    if let Some(icon_name) = icon {
                        html! {
                            <Icon icon={icon_name} />
                        }
                    } else if let Some(html) = icon_html.clone() {
                        html
                    } else {
                        html! {
                            <Icon icon={IconName::Blank} />
                        }
                    }
                }
                <div class={classes!("bp3-text", "bp3-fill", text_class.clone())}>
                    {text.clone()}
                </div>
                {
                    if let Some(label) = label.clone() {
                        html! {
                            <span
                                class={classes!(
                                    "bp3-menu-item-label",
                                    label_class.clone())}
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
pub fn menu_divider(props: &MenuDividerProps) -> Html {
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
