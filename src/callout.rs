use crate::icon::IconSize;
use crate::{Icon, Intent};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct CalloutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(false)]
    pub without_icon: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    pub children: Children,
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
    } else if let Some(icon) = icon.clone() {
        Some(icon)
    } else if let Some(intent) = intent {
        Some(match intent {
            Intent::Primary => Icon::InfoSign,
            Intent::Success => Icon::Tick,
            Intent::Warning => Icon::WarningSign,
            Intent::Danger => Icon::Error,
        })
    } else {
        None
    };

    html! {
        <div
            class={classes!(
                "bp3-callout",
                icon.is_some().then_some("bp3-callout-icon"),
                intent,
                class.clone(),
            )}
        >
            <Icon {icon} size={IconSize::LARGE} />
            {
                title.iter()
                    .map(|title| html!{<h4 class={"bp3-heading"}>{title}</h4>})
                    .collect::<Html>()
            }
            {children.clone()}
        </div>
    }
}
