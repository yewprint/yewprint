use crate::Intent;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProgressBarProps {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub stripes: bool,
    #[prop_or_default]
    pub value: Option<f32>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ProgressBar)]
pub fn progress_bar(
    ProgressBarProps {
        animate,
        stripes,
        value,
        intent,
        class,
    }: &ProgressBarProps,
) -> Html {
    let style = if let Some(value) = value {
        // NOTE: nightly, issue #44095 for f32::clamp
        // let percent = ((1000. * value).ceil() / 10.).clamp(0.,100.);
        let percent = ((1000. * value).ceil() / 10.).max(0.).min(100.);
        AttrValue::from(format!("width: {}%;", percent))
    } else {
        "".into()
    };

    html! {
        <div
            class={classes!(
                "bp3-progress-bar",
                intent,
                (!animate).then_some("bp3-no-animation"),
                (!stripes).then_some("bp3-no-stripes"),
                class.clone(),
            )}
        >
            <div class={classes!("bp3-progress-meter")} {style}/>
        </div>
    }
}
