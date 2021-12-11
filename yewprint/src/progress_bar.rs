use crate::Intent;
use yew::prelude::*;

pub struct ProgressBar {
    props: ProgressBarProps,
}

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

impl Component for ProgressBar {
    type Message = ();
    type Properties = ProgressBarProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let width = if let Some(value) = self.props.value {
            // NOTE: nightly, issue #44095 for f32::clamp
            // let percent = ((1000. * value).ceil() / 10.).clamp(0.,100.);
            let percent = ((1000. * value).ceil() / 10.).max(0.).min(100.);
            format!("width: {}%;", percent)
        } else {
            "".into()
        };
        html! {
            <div
                class=classes!(
                    "bp3-progress-bar",
                    self.props.intent,
                    (!self.props.animate).then(|| "bp3-no-animation"),
                    (!self.props.stripes).then(|| "bp3-no-stripes"),
                    self.props.class.clone(),
                )
            >
                <div class=classes!("bp3-progress-meter") style={{width}}/>
            </div>
        }
    }
}
