use crate::icon::ICON_SIZE_LARGE;
use crate::{Icon, IconName, Intent};
use yew::prelude::*;

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
    pub title: Option<String>,
    pub children: html::Children,
}

#[function_component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    let icon = if props.without_icon {
        None
    } else {
        props.icon.or_else(|| {
            props.intent.map(|intent| match intent {
                Intent::Primary => IconName::InfoSign,
                Intent::Success => IconName::Tick,
                Intent::Warning => IconName::WarningSign,
                Intent::Danger => IconName::Error,
            })
        })
    };
    let classes = classes!(
        props.class.clone(),
        "bp3-callout",
        icon.map(|_| "bp3-callout-icon"),
        props.intent,
    );
    html! {
        <div class={classes}>
            {
                icon.iter()
                    .map(|name| html!{<Icon icon={*name} icon_size={ICON_SIZE_LARGE}/>})
                    .collect::<Html>()
            }
            {
                props.title.iter()
                    .map(|title| html!{<h4 class={"bp3-heading"}>{title}</h4>})
                    .collect::<Html>()
            }
            { props.children.clone() }
        </div>
    }
}
