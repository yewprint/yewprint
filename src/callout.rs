use crate::icon::ICON_SIZE_LARGE;
use crate::{Icon, IconName, Intent};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct CalloutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub without_icon: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    pub children: html::Children,
}

#[function_component(Callout)]
pub fn callout(
    CalloutProps {
        class,
        without_icon,
        icon,
        intent,
        title,
        children,
    }: &CalloutProps,
) -> Html {
    let icon = if *without_icon {
        None
    } else {
        icon.or_else(|| {
            intent.map(|intent| match intent {
                Intent::Primary => IconName::InfoSign,
                Intent::Success => IconName::Tick,
                Intent::Warning => IconName::WarningSign,
                Intent::Danger => IconName::Error,
            })
        })
    };

    html! {
        <div
            class={classes!(
                "bp3-callout",
                icon.map(|_| "bp3-callout-icon"),
                intent,
                class.clone(),
            )}
        >
            {
                icon.iter()
                    .map(|icon| html!{<Icon {icon} icon_size={ICON_SIZE_LARGE}/>})
                    .collect::<Html>()
            }
            {
                title.iter()
                    .map(|title| html!{<h4 class={"bp3-heading"}>{title}</h4>})
                    .collect::<Html>()
            }
            {children.clone()}
        </div>
    }
}
