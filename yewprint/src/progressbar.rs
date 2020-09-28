use crate::Intent;
use yew::prelude::*;

pub struct ProgressBar {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub animate: bool,
    #[prop_or(false)]
    pub stripes: bool,
    #[prop_or_default]
    pub value: Option<f32>,
    #[prop_or_default]
    pub intent: Option<Intent>,
}

impl Component for ProgressBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
        let width = if let Some(value) = self.props.value {
            //let percent = ((1000. * value).ceil() / 10.).clamp(0.,100.);
            let percent = ((1000. * value).ceil() / 10.).max(0.).min(100.);
            format!("width: {}%;", percent)
        } else {
            "".into()
        };
        html! {
            <div class=("bp3-progress-bar", self.props.intent, Some("bp3-no-animation").filter(|_| !self.props.animate.clone()), Some("bp3-no-stripes").filter(|_| !self.props.stripes.clone()))>
                <div class="bp3-progress-meter" style={{width}}/>
            </div>
        }
    }
}
