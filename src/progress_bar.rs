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
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let width = if let Some(value) = props.value {
        // NOTE: nightly, issue #44095 for f32::clamp
        // let percent = ((1000. * value).ceil() / 10.).clamp(0.,100.);
        let percent = ((1000. * value).ceil() / 10.).max(0.).min(100.);
        format!("width: {}%;", percent)
    } else {
        "".into()
    };
    html! {
        <div
            class={classes!(
                "bp3-progress-bar",
                props.intent,
                (!props.animate).then(|| "bp3-no-animation"),
                (!props.stripes).then(|| "bp3-no-stripes"),
                props.class.clone(),
            )}
        >
            <div class={classes!("bp3-progress-meter")} style={{width}}/>
        </div>
    }
}
